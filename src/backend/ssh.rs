use std::{borrow::Cow, path::PathBuf, sync::Arc};

use anyhow::{Context, Result, anyhow};
use rand::RngCore;
use russh::{
    AlgorithmKind, Channel, ChannelMsg, ChannelOpenFailure, Disconnect, Error as RusshError,
    Preferred, cipher,
    client::{self, Handler, Msg},
    kex,
    keys::ssh_key::Algorithm,
};
#[cfg(unix)]
use tokio::net::UnixStream;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc,
    time::{Duration, sleep},
};

use crate::{
    backend::auth::{load_session_private_key, private_keys_with_algs},
    session::config::{AuthMethod, ConfigStore, Session},
    system::{SystemSnapshot, remote_snapshot_from_kv},
    terminal::{BackendCommand, BackendEvent, BackendTx},
};

pub fn spawn_ssh_terminal(
    runtime: &tokio::runtime::Handle,
    tab_id: String,
    session: Session,
    cols: u16,
    rows: u16,
    events: std::sync::mpsc::Sender<BackendEvent>,
) -> BackendTx {
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<BackendCommand>();
    let task_tab = tab_id.clone();
    runtime.spawn(async move {
        if let Err(err) = run_ssh(
            task_tab.clone(),
            session,
            cols,
            rows,
            cmd_rx,
            events.clone(),
        )
        .await
        {
            let _ = events.send(BackendEvent::Closed {
                tab_id: task_tab,
                reason: format!("{err:#}"),
            });
        }
    });
    BackendTx::Ssh(cmd_tx)
}

async fn sample_remote_system_with_handle(
    handle: Arc<tokio::sync::Mutex<russh::client::Handle<ClientHandler>>>,
) -> Result<SystemSnapshot> {
    let mut channel = handle
        .lock()
        .await
        .channel_open_session()
        .await
        .context("open metrics session")?;
    channel
        .exec(true, REMOTE_SYSTEM_PROBE)
        .await
        .context("exec remote metrics probe")?;

    let mut stdout = Vec::new();
    while let Some(msg) = channel.wait().await {
        match msg {
            ChannelMsg::Data { data } | ChannelMsg::ExtendedData { data, ext: _ } => {
                stdout.extend_from_slice(&data);
            }
            ChannelMsg::Close => break,
            _ => {}
        }
    }

    let output = String::from_utf8_lossy(&stdout);
    remote_snapshot_from_kv(&output)
}

async fn run_ssh(
    tab_id: String,
    session: Session,
    cols: u16,
    rows: u16,
    mut commands: mpsc::UnboundedReceiver<BackendCommand>,
    events: std::sync::mpsc::Sender<BackendEvent>,
) -> Result<()> {
    let config = ConfigStore::load().unwrap_or_else(|_| ConfigStore::in_memory());
    let x11 = X11ForwardingState::from_config(&config);
    let _ = events.send(BackendEvent::Status {
        tab_id: tab_id.clone(),
        text: format!(
            "connecting {}@{}:{}...",
            session.user, session.host, session.port
        ),
    });

    let handle = Arc::new(tokio::sync::Mutex::new(
        connect_and_authenticate(&tab_id, &session, &events, x11.clone()).await?,
    ));

    let mut channel = handle
        .lock()
        .await
        .channel_open_session()
        .await
        .context("open session")?;
    channel
        .request_pty(true, "xterm-256color", cols.into(), rows.into(), 0, 0, &[])
        .await
        .context("request pty")?;
    if let Some(x11) = x11.as_ref() {
        match channel
            .request_x11(
                true,
                false,
                "MIT-MAGIC-COOKIE-1",
                x11.fake_cookie_hex.clone(),
                x11.screen_number,
            )
            .await
        {
            Ok(()) => {
                let _ = channel
                    .set_env(false, "DISPLAY", x11.remote_display.clone())
                    .await;
                let _ = events.send(BackendEvent::Status {
                    tab_id: tab_id.clone(),
                    text: format!("X11 forwarding requested, DISPLAY={}", x11.remote_display),
                });
            }
            Err(err) => {
                tracing::warn!("[ssh] X11 forwarding request failed: {err}");
                let _ = events.send(BackendEvent::Status {
                    tab_id: tab_id.clone(),
                    text: format!("X11 forwarding unavailable: {err}"),
                });
            }
        }
    }
    channel.request_shell(true).await.context("request shell")?;

    let _ = events.send(BackendEvent::Status {
        tab_id: tab_id.clone(),
        text: format!("connected {}@{}", session.user, session.host),
    });
    let _ = events.send(BackendEvent::Connected {
        tab_id: tab_id.clone(),
    });

    let exit_reason;
    let mut is_graceful_close = false;

    loop {
        tokio::select! {
            command = commands.recv() => {
                match command {
                    Some(BackendCommand::Input(bytes)) => {
                        if let Err(err) = channel.data(bytes.as_slice()).await {
                            tracing::error!("[ssh] write error on tab {}: {}", tab_id, err);
                            exit_reason = format!("ssh write error: {err}");
                            break;
                        }
                    }
                    Some(BackendCommand::Resize { cols, rows }) => {
                        let _ = channel.window_change(cols.into(), rows.into(), 0, 0).await;
                    }
                    Some(BackendCommand::SampleMetrics) => {
                        let handle_clone = handle.clone();
                        let tab_id_clone = tab_id.clone();
                        let events_clone = events.clone();
                        tokio::spawn(async move {
                            match sample_remote_system_with_handle(handle_clone).await {
                                Ok(snapshot) => {
                                    let _ = events_clone.send(BackendEvent::RemoteSystem {
                                        tab_id: tab_id_clone,
                                        snapshot,
                                    });
                                }
                                Err(err) => {
                                    let _ = events_clone.send(BackendEvent::RemoteSystemUnavailable {
                                        tab_id: tab_id_clone,
                                        reason: format!("remote metrics unavailable: {err:#}"),
                                    });
                                }
                            }
                        });
                    }
                    Some(BackendCommand::Close) | None => {
                        tracing::info!("[ssh] local client closed the session for tab {}", tab_id);
                        let _ = channel.eof().await;
                        exit_reason = "ssh session closed".to_string();
                        break;
                    }
                }
            }
            msg = channel.wait() => {
                match msg {
                    Some(ChannelMsg::Data { data }) | Some(ChannelMsg::ExtendedData { data, ext: _ }) => {
                        let _ = events.send(BackendEvent::Output {
                            tab_id: tab_id.clone(),
                            bytes: data.to_vec(),
                        });
                    }
                    Some(ChannelMsg::ExitStatus { exit_status: _ }) | Some(ChannelMsg::Eof) => {
                        is_graceful_close = true;
                    }
                    Some(ChannelMsg::Close) => {
                        if is_graceful_close {
                            tracing::info!("[ssh] session gracefully closed by server for tab {}", tab_id);
                            exit_reason = "ssh session closed".to_string();
                        } else {
                            tracing::warn!("[ssh] connection abruptly closed by server for tab {}", tab_id);
                            exit_reason = "ssh connection lost (abrupt close)".to_string();
                        }
                        break;
                    }
                    None => {
                        if is_graceful_close {
                            tracing::info!("[ssh] network stream ended gracefully for tab {}", tab_id);
                            exit_reason = "ssh session closed".to_string();
                        } else {
                            tracing::warn!("[ssh] network drop detected for tab {}", tab_id);
                            exit_reason = "ssh connection lost (network drop)".to_string();
                        }
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    let _ = handle
        .lock()
        .await
        .disconnect(Disconnect::ByApplication, "bye", "")
        .await;
    let _ = events.send(BackendEvent::Closed {
        tab_id,
        reason: exit_reason,
    });
    Ok(())
}

async fn connect_and_authenticate(
    tab_id: &str,
    session: &Session,
    events: &std::sync::mpsc::Sender<BackendEvent>,
    x11: Option<Arc<X11ForwardingState>>,
) -> Result<russh::client::Handle<ClientHandler>> {
    let addr = format!("{}:{}", session.host, session.port);
    tracing::info!(
        "[ssh] initiating tcp connection to {} (user: {})",
        addr,
        session.user
    );
    let status_text =
        if let Some((ptype, phost, pport)) = crate::session::config::active_proxy(session) {
            let pport_val = pport.unwrap_or_else(|| if ptype == "http" { 8080 } else { 1080 });
            format!(
                "connecting to {addr} via {} proxy {}:{}",
                ptype.to_uppercase(),
                phost,
                pport_val
            )
        } else {
            format!("opening tcp connection to {addr}")
        };
    let _ = events.send(BackendEvent::Status {
        tab_id: tab_id.to_string(),
        text: status_text,
    });

    let mut handle = match connect_with_mode(
        session,
        &addr,
        SshCompatibilityMode::Default,
        x11.clone(),
    )
    .await
    {
        Ok(handle) => handle,
        Err(default_err) => {
            let Some(default_details) = negotiation_error_details(&default_err) else {
                return Err(default_err);
            };
            tracing::warn!(
                "[ssh] default negotiation failed for {}@{}: {}",
                session.user,
                addr,
                default_details
            );
            let short_reason = negotiation_error_short_reason(&default_err)
                .unwrap_or_else(|| "algorithm mismatch".to_string());
            let _ = events.send(BackendEvent::Status {
                tab_id: tab_id.to_string(),
                text: format!(
                    "default SSH negotiation failed ({short_reason}), retrying legacy compatibility algorithms"
                ),
            });

            match connect_with_mode(session, &addr, SshCompatibilityMode::Legacy, x11.clone()).await
            {
                Ok(handle) => {
                    tracing::warn!(
                        "[ssh] connected to {} using legacy SSH compatibility mode",
                        addr
                    );
                    let _ = events.send(BackendEvent::Status {
                        tab_id: tab_id.to_string(),
                        text: format!("connected to {addr} using legacy SSH compatibility mode"),
                    });
                    handle
                }
                Err(legacy_err) => {
                    let legacy_details = negotiation_error_details(&legacy_err)
                        .unwrap_or_else(|| format!("{legacy_err:#}"));
                    tracing::error!(
                        "[ssh] legacy compatibility negotiation failed for {}@{}: {}",
                        session.user,
                        addr,
                        legacy_details
                    );
                    return Err(anyhow!(
                        "SSH negotiation failed. default mode: {default_details}. legacy compatibility mode: {legacy_details}"
                    ));
                }
            }
        }
    };

    tracing::debug!("[ssh] tcp connected to {}", addr);

    let authed = match session.auth {
        AuthMethod::Password => {
            tracing::info!(
                "[ssh] sending password authentication for {}@{}",
                session.user,
                addr
            );
            let _ = events.send(BackendEvent::Status {
                tab_id: tab_id.to_string(),
                text: format!(
                    "connected to {addr}, sending password authentication for {}",
                    session.user
                ),
            });
            handle
                .authenticate_password(&session.user, &session.password)
                .await
                .context("password authentication failed")?
                .success()
        }
        AuthMethod::Key => {
            let source = key_source_label(session);
            tracing::info!(
                "[ssh] sending key authentication for {}@{} (key source: {})",
                session.user,
                addr,
                source
            );
            let _ = events.send(BackendEvent::Status {
                tab_id: tab_id.to_string(),
                text: format!("connected to {addr}, loading private key from {source}"),
            });
            let keypair = load_session_private_key(session)?;
            let algorithm = format!("{:?}", keypair.algorithm());
            let _ = events.send(BackendEvent::Status {
                tab_id: tab_id.to_string(),
                text: format!("private key loaded from {source}, algorithm {algorithm}, sending public key authentication for {}", session.user),
            });
            let keys = private_keys_with_algs(keypair).context("invalid private key")?;
            let mut success = false;
            for key in keys {
                match handle.authenticate_publickey(&session.user, key).await {
                    Ok(result) if result.success() => {
                        success = true;
                        break;
                    }
                    Ok(_) => {
                        tracing::debug!("[ssh] public key auth failed with algorithm, trying next");
                        continue;
                    }
                    Err(e) => {
                        tracing::debug!("[ssh] public key auth error: {:?}, trying next", e);
                        continue;
                    }
                }
            }
            if !success {
                return Err(anyhow::anyhow!(
                    "public key authentication failed for {}@{}:{} using {} ({})",
                    session.user,
                    session.host,
                    session.port,
                    source,
                    algorithm
                ));
            }
            success
        }
    };

    if !authed {
        tracing::warn!("[ssh] authentication failed for {}@{}", session.user, addr);
        let _ = handle
            .disconnect(Disconnect::ByApplication, "auth failed", "")
            .await;
        return Err(anyhow!(
            "{}",
            match session.auth {
                AuthMethod::Password => format!(
                    "authentication failed: server rejected password authentication for {}@{}:{}",
                    session.user, session.host, session.port
                ),
                AuthMethod::Key => format!(
                    "authentication failed: server rejected public key authentication for {}@{}:{} using {}",
                    session.user,
                    session.host,
                    session.port,
                    key_source_label(session)
                ),
            }
        ));
    }

    tracing::info!(
        "[ssh] authentication successful for {}@{}",
        session.user,
        addr
    );

    let _ = events.send(BackendEvent::Status {
        tab_id: tab_id.to_string(),
        text: format!(
            "authentication accepted, opening shell for {}@{}",
            session.user, session.host
        ),
    });

    Ok(handle)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SshCompatibilityMode {
    Default,
    Legacy,
}

impl SshCompatibilityMode {
    fn label(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Legacy => "legacy compatibility",
        }
    }
}

async fn connect_with_mode(
    session: &Session,
    addr: &str,
    mode: SshCompatibilityMode,
    x11: Option<Arc<X11ForwardingState>>,
) -> Result<russh::client::Handle<ClientHandler>> {
    let stream = crate::session::config::connect_proxy(session).await?;
    client::connect_stream(
        Arc::new(ssh_client_config(mode)),
        stream,
        ClientHandler::new(x11),
    )
    .await
    .with_context(|| format!("connect {addr} failed in {} mode", mode.label()))
}

fn ssh_client_config(mode: SshCompatibilityMode) -> client::Config {
    let mut config = client::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(600)),
        keepalive_interval: Some(std::time::Duration::from_secs(3)),
        keepalive_max: 2,
        ..Default::default()
    };
    if mode == SshCompatibilityMode::Legacy {
        config.preferred = legacy_ssh_preferred();
    }
    config
}

fn legacy_ssh_preferred() -> Preferred {
    let mut preferred = Preferred::default();

    let mut kex_order = preferred.kex.iter().cloned().collect::<Vec<_>>();
    extend_unique(
        &mut kex_order,
        [
            kex::ECDH_SHA2_NISTP256,
            kex::ECDH_SHA2_NISTP384,
            kex::ECDH_SHA2_NISTP521,
            kex::DH_G14_SHA1,
            kex::DH_G1_SHA1,
        ],
    );
    preferred.kex = Cow::Owned(kex_order);

    let mut key_order = preferred.key.iter().cloned().collect::<Vec<_>>();
    extend_unique(&mut key_order, [Algorithm::Dsa]);
    preferred.key = Cow::Owned(key_order);

    let mut cipher_order = preferred.cipher.iter().cloned().collect::<Vec<_>>();
    extend_unique(
        &mut cipher_order,
        [
            cipher::AES_128_CBC,
            cipher::AES_192_CBC,
            cipher::AES_256_CBC,
            cipher::TRIPLE_DES_CBC,
        ],
    );
    preferred.cipher = Cow::Owned(cipher_order);

    preferred
}

fn extend_unique<T>(items: &mut Vec<T>, extras: impl IntoIterator<Item = T>)
where
    T: PartialEq,
{
    for item in extras {
        if !items.contains(&item) {
            items.push(item);
        }
    }
}

fn negotiation_error_short_reason(err: &anyhow::Error) -> Option<String> {
    match russh_error_from_anyhow(err)? {
        RusshError::NoCommonAlgo { kind, .. } => Some(format!(
            "no common {} algorithm",
            algorithm_kind_label(kind)
        )),
        _ => None,
    }
}

fn negotiation_error_details(err: &anyhow::Error) -> Option<String> {
    match russh_error_from_anyhow(err)? {
        RusshError::NoCommonAlgo { kind, ours, theirs } => Some(format!(
            "no common {} algorithm; client offers [{}]; server offers [{}]",
            algorithm_kind_label(kind),
            ours.join(", "),
            theirs.join(", ")
        )),
        _ => None,
    }
}

fn russh_error_from_anyhow(err: &anyhow::Error) -> Option<&RusshError> {
    err.chain()
        .find_map(|cause| cause.downcast_ref::<RusshError>())
}

fn algorithm_kind_label(kind: &AlgorithmKind) -> &'static str {
    match kind {
        AlgorithmKind::Kex => "KEX",
        AlgorithmKind::Key => "host key",
        AlgorithmKind::Cipher => "cipher",
        AlgorithmKind::Compression => "compression",
        AlgorithmKind::Mac => "MAC",
    }
}

fn key_source_label(session: &Session) -> String {
    let path = session.private_key_path.trim();
    let has_inline = !session.private_key_inline.trim().is_empty();
    match (!path.is_empty(), has_inline) {
        (true, true) => format!("inline key or {}", path),
        (true, false) => path.to_string(),
        (false, true) => "inline key text".to_string(),
        (false, false) => "unknown key source".to_string(),
    }
}

const REMOTE_SYSTEM_PROBE: &str = r#"sh -lc '
os=$(uname -s 2>/dev/null || echo unknown)

if [ "$os" = "Linux" ] && [ -r /proc/stat ]; then
  cpu_stat() { awk '"'"'/^cpu / { print ($2+$3+$4+$5+$6+$7+$8), $5 }'"'"' /proc/stat 2>/dev/null; }
  net_stat() { awk -F"[: ]+" '"'"'/:/ && $1!="Inter" && $1!="face" { rx += $3; tx += $11 } END { print rx+0, tx+0 }'"'"' /proc/net/dev 2>/dev/null; }

  read cpu_total_1 cpu_idle_1 <<EOF
$(cpu_stat)
EOF
  read net_rx_1 net_tx_1 <<EOF
$(net_stat)
EOF
  sleep 1
  read cpu_total_2 cpu_idle_2 <<EOF
$(cpu_stat)
EOF
  read net_rx_2 net_tx_2 <<EOF
$(net_stat)
EOF

  cpu_delta=$((cpu_total_2 - cpu_total_1))
  idle_delta=$((cpu_idle_2 - cpu_idle_1))
  cpu_percent=$(awk -v total="$cpu_delta" -v idle="$idle_delta" '"'"'BEGIN { if (total <= 0) print "0.00"; else printf "%.2f", ((total-idle)/total)*100 }'"'"')
  mem_total=$(awk '"'"'/^MemTotal:/ {print $2 * 1024}'"'"' /proc/meminfo 2>/dev/null)
  mem_available=$(awk '"'"'/^MemAvailable:/ {print $2 * 1024}'"'"' /proc/meminfo 2>/dev/null)
  swap_total=$(awk '"'"'/^SwapTotal:/ {print $2 * 1024}'"'"' /proc/meminfo 2>/dev/null)
  swap_free=$(awk '"'"'/^SwapFree:/ {print $2 * 1024}'"'"' /proc/meminfo 2>/dev/null)

  echo "CPU_PERCENT=${cpu_percent:-0.00}"
  echo "MEM_TOTAL=${mem_total:-0}"
  echo "MEM_USED=$(( ${mem_total:-0} - ${mem_available:-0} ))"
  echo "SWAP_TOTAL=${swap_total:-0}"
  echo "SWAP_USED=$(( ${swap_total:-0} - ${swap_free:-0} ))"
  echo "NET_RX=$(( ${net_rx_2:-0} - ${net_rx_1:-0} ))"
  echo "NET_TX=$(( ${net_tx_2:-0} - ${net_tx_1:-0} ))"
  df -kP 2>/dev/null | awk "NR > 1 && \$1 !~ /^(tmpfs|devtmpfs|ramfs|overlay|aufs)\$/ { printf \"DISK=%s\t%s\t%s\n\", \$6, \$4 * 1024, \$2 * 1024 }" | head -n 6
  exit 0
fi

if [ "$os" = "Darwin" ]; then
  net_stat() { netstat -ibn 2>/dev/null | awk '"'"'NR > 1 && $7 ~ /^[0-9]+$/ && $10 ~ /^[0-9]+$/ { rx += $7; tx += $10 } END { print rx+0, tx+0 }'"'"'; }

  read net_rx_1 net_tx_1 <<EOF
$(net_stat)
EOF
  sleep 1
  read net_rx_2 net_tx_2 <<EOF
$(net_stat)
EOF

  cpu_percent=$(top -l 2 -n 0 -s 1 2>/dev/null | awk -F"[:,% ]+" '"'"'/CPU usage:/ { user=$3; sys=$5 } END { if (user == "" && sys == "") print "0.00"; else printf "%.2f", user + sys }'"'"')
  mem_total=$(sysctl -n hw.memsize 2>/dev/null || echo 0)
  pagesize=$(sysctl -n hw.pagesize 2>/dev/null || echo 4096)
  vm_output=$(vm_stat 2>/dev/null)
  pages_active=$(printf "%s\n" "$vm_output" | awk '"'"'/Pages active/ { gsub("\\.","",$3); print $3+0 }'"'"')
  pages_wired=$(printf "%s\n" "$vm_output" | awk '"'"'/Pages wired down/ { gsub("\\.","",$4); print $4+0 }'"'"')
  pages_compressed=$(printf "%s\n" "$vm_output" | awk '"'"'/Pages occupied by compressor/ { gsub("\\.","",$5); print $5+0 }'"'"')
  pages_speculative=$(printf "%s\n" "$vm_output" | awk '"'"'/Pages speculative/ { gsub("\\.","",$3); print $3+0 }'"'"')
  mem_used=$(( (${pages_active:-0} + ${pages_wired:-0} + ${pages_compressed:-0} + ${pages_speculative:-0}) * ${pagesize:-4096} ))
  swap_line=$(sysctl vm.swapusage 2>/dev/null || true)
  swap_used=$(printf "%s\n" "$swap_line" | awk -F"[= ,]+" '"'"'
    function mult(unit) { return unit=="K"?1024:(unit=="M"?1048576:(unit=="G"?1073741824:(unit=="T"?1099511627776:1))) }
    /used/ { value=$4; unit=substr(value, length(value), 1); sub(/[A-Za-z]+$/, "", value); printf "%.0f", value * mult(unit) }'"'"')
  swap_total=$(printf "%s\n" "$swap_line" | awk -F"[= ,]+" '"'"'
    function mult(unit) { return unit=="K"?1024:(unit=="M"?1048576:(unit=="G"?1073741824:(unit=="T"?1099511627776:1))) }
    /used/ && /free/ { used=$4; free=$8; unit1=substr(used, length(used), 1); unit2=substr(free, length(free), 1); sub(/[A-Za-z]+$/, "", used); sub(/[A-Za-z]+$/, "", free); printf "%.0f", (used * mult(unit1)) + (free * mult(unit2)) }'"'"')

  echo "CPU_PERCENT=${cpu_percent:-0.00}"
  echo "MEM_TOTAL=${mem_total:-0}"
  echo "MEM_USED=${mem_used:-0}"
  echo "SWAP_TOTAL=${swap_total:-0}"
  echo "SWAP_USED=${swap_used:-0}"
  echo "NET_RX=$(( ${net_rx_2:-0} - ${net_rx_1:-0} ))"
  echo "NET_TX=$(( ${net_tx_2:-0} - ${net_tx_1:-0} ))"
  df -kP 2>/dev/null | awk "NR > 1 && \$1 !~ /^(devfs|tmpfs|devtmpfs|ramfs|overlay|aufs)\$/ { printf \"DISK=%s\t%s\t%s\n\", \$6, \$4 * 1024, \$2 * 1024 }" | head -n 6
  exit 0
fi

echo "CPU_PERCENT=0.00"
echo "MEM_TOTAL=0"
echo "MEM_USED=0"
echo "SWAP_TOTAL=0"
echo "SWAP_USED=0"
echo "NET_RX=0"
echo "NET_TX=0"
'"#;

const X11_AUTH_PROTOCOL: &str = "MIT-MAGIC-COOKIE-1";
const X11_REMOTE_DISPLAY: &str = "localhost:10.0";
const X11_DEFAULT_SCREEN: u32 = 0;
const X11_MAX_AUTH_FIELD_LEN: usize = 4096;

trait AsyncReadWrite: AsyncRead + AsyncWrite + Unpin + Send {}

impl<T> AsyncReadWrite for T where T: AsyncRead + AsyncWrite + Unpin + Send {}

#[derive(Clone)]
struct X11ForwardingState {
    fake_cookie: [u8; 16],
    fake_cookie_hex: String,
    local_display: String,
    remote_display: String,
    screen_number: u32,
    launch_local_x_server: bool,
    local_x_server_app_path: String,
}

impl X11ForwardingState {
    fn from_config(config: &ConfigStore) -> Option<Arc<Self>> {
        if !config.x11_forwarding_enabled() {
            return None;
        }

        let mut fake_cookie = [0u8; 16];
        rand::rngs::OsRng.fill_bytes(&mut fake_cookie);
        let local_display = crate::session::config::default_local_x_display();

        Some(Arc::new(Self {
            fake_cookie,
            fake_cookie_hex: hex::encode(fake_cookie),
            local_display,
            remote_display: X11_REMOTE_DISPLAY.to_string(),
            screen_number: X11_DEFAULT_SCREEN,
            launch_local_x_server: config.x11_launch_local_x_server(),
            local_x_server_app_path: config.local_x_server_app_path().to_string(),
        }))
    }
}

struct LocalX11Auth {
    protocol: Vec<u8>,
    data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum X11Endpoint {
    Unix(PathBuf),
    Tcp(String, u16),
}

async fn prepare_x11_relay(
    x11: &X11ForwardingState,
) -> Result<(LocalX11Auth, Box<dyn AsyncReadWrite>)> {
    if x11.launch_local_x_server {
        match crate::app::startup::launch_local_x_server_app(&x11.local_x_server_app_path) {
            Ok(()) => sleep(Duration::from_millis(700)).await,
            Err(err) => tracing::warn!("[ssh:x11] failed to launch configured X server: {err:#}"),
        }
    }

    let auth = match load_local_x11_cookie(&x11.local_display) {
        Ok(cookie) => LocalX11Auth {
            protocol: X11_AUTH_PROTOCOL.as_bytes().to_vec(),
            data: cookie,
        },
        Err(err) => {
            tracing::warn!(
                "[ssh:x11] no local X11 MIT cookie found for {}; falling back to no-auth setup: {err:#}",
                x11.local_display
            );
            LocalX11Auth {
                protocol: Vec::new(),
                data: Vec::new(),
            }
        }
    };
    let local_x = connect_local_x11(&x11.local_display)
        .await
        .with_context(|| format!("connect local X server at {}", x11.local_display))?;
    Ok((auth, local_x))
}

async fn connect_local_x11(display: &str) -> Result<Box<dyn AsyncReadWrite>> {
    let endpoints = local_x11_endpoints(display);
    if endpoints.is_empty() {
        return Err(anyhow!(
            "no local X11 endpoint candidates for DISPLAY={display}"
        ));
    }

    let mut errors = Vec::new();
    for attempt in 0..5 {
        for endpoint in &endpoints {
            match connect_x11_endpoint(endpoint).await {
                Ok(stream) => return Ok(stream),
                Err(err) => errors.push(format!("{endpoint:?}: {err}")),
            }
        }
        if attempt < 4 {
            sleep(Duration::from_millis(250)).await;
        }
    }

    Err(anyhow!(
        "unable to connect local X11 server; tried {}; {}",
        endpoints
            .iter()
            .map(|endpoint| format!("{endpoint:?}"))
            .collect::<Vec<_>>()
            .join(", "),
        errors.join("; ")
    ))
}

async fn connect_x11_endpoint(endpoint: &X11Endpoint) -> Result<Box<dyn AsyncReadWrite>> {
    match endpoint {
        X11Endpoint::Unix(path) => {
            #[cfg(unix)]
            {
                let stream = UnixStream::connect(path).await?;
                Ok(Box::new(stream))
            }
            #[cfg(not(unix))]
            {
                let _ = path;
                Err(anyhow!(
                    "Unix X11 sockets are not supported on this platform"
                ))
            }
        }
        X11Endpoint::Tcp(host, port) => {
            let stream = TcpStream::connect((host.as_str(), *port)).await?;
            Ok(Box::new(stream))
        }
    }
}

fn local_x11_endpoints(display: &str) -> Vec<X11Endpoint> {
    let display = display.trim();
    let display = if display.is_empty() { ":0" } else { display };
    let mut endpoints = Vec::new();

    if let Some(path) = launchd_xquartz_socket_path(display) {
        endpoints.push(X11Endpoint::Unix(path));
    }

    if let Some(display_number) = x11_display_number(display) {
        if !cfg!(target_os = "windows")
            && (display.starts_with(':')
                || display.starts_with('/')
                || display.starts_with("unix:"))
        {
            endpoints.push(X11Endpoint::Unix(PathBuf::from(format!(
                "/tmp/.X11-unix/X{display_number}"
            ))));
            endpoints.push(X11Endpoint::Unix(PathBuf::from(format!(
                "/private/tmp/.X11-unix/X{display_number}"
            ))));
        }

        let host = x11_display_host(display);
        let port = 6000u16.saturating_add(display_number);
        match host.as_deref() {
            Some("unix") | None => endpoints.push(X11Endpoint::Tcp("127.0.0.1".to_string(), port)),
            Some(host) if !host.is_empty() => {
                endpoints.push(X11Endpoint::Tcp(host.to_string(), port))
            }
            _ => endpoints.push(X11Endpoint::Tcp("127.0.0.1".to_string(), port)),
        }
    }

    endpoints.dedup();
    endpoints
}

fn launchd_xquartz_socket_path(display: &str) -> Option<PathBuf> {
    if !display.starts_with('/') {
        return None;
    }
    let (path, _) = display.rsplit_once(':')?;
    (!path.is_empty()).then(|| PathBuf::from(path))
}

fn x11_display_number(display: &str) -> Option<u16> {
    let (_, rest) = display.rsplit_once(':')?;
    let number = rest.split('.').next().unwrap_or(rest);
    number.parse::<u16>().ok()
}

fn x11_display_host(display: &str) -> Option<String> {
    if display.starts_with('/') {
        return None;
    }
    display
        .rsplit_once(':')
        .map(|(host, _)| host.trim_start_matches("tcp/").to_string())
        .filter(|host| !host.is_empty())
}

fn load_local_x11_cookie(display: &str) -> Result<Vec<u8>> {
    let mut errors = Vec::new();
    for xauth in xauth_candidates() {
        for candidate in xauth_display_candidates(display) {
            match std::process::Command::new(&xauth)
                .arg("list")
                .arg(&candidate)
                .output()
            {
                Ok(output) if output.status.success() => {
                    if let Some(cookie) = parse_xauth_cookie(&output.stdout) {
                        return Ok(cookie);
                    }
                }
                Ok(output) => errors.push(format!(
                    "{} list {} exited with {}",
                    xauth.display(),
                    candidate,
                    output.status
                )),
                Err(err) => errors.push(format!("{}: {err}", xauth.display())),
            }
        }

        match std::process::Command::new(&xauth).arg("list").output() {
            Ok(output) if output.status.success() => {
                if let Some(cookie) = parse_xauth_cookie(&output.stdout) {
                    return Ok(cookie);
                }
            }
            Ok(output) => errors.push(format!(
                "{} list exited with {}",
                xauth.display(),
                output.status
            )),
            Err(err) => errors.push(format!("{}: {err}", xauth.display())),
        }
    }

    Err(anyhow!(
        "no MIT-MAGIC-COOKIE-1 entry found via xauth; {}",
        errors.join("; ")
    ))
}

fn xauth_candidates() -> Vec<PathBuf> {
    let mut candidates = vec![PathBuf::from("xauth")];
    candidates.push(PathBuf::from("/opt/X11/bin/xauth"));
    candidates.push(PathBuf::from("/usr/X11/bin/xauth"));
    #[cfg(target_os = "windows")]
    {
        if let Ok(program_files) = std::env::var("ProgramFiles") {
            candidates.push(
                PathBuf::from(&program_files)
                    .join("VcXsrv")
                    .join("xauth.exe"),
            );
            candidates.push(
                PathBuf::from(&program_files)
                    .join("Xming")
                    .join("xauth.exe"),
            );
        }
        if let Ok(program_files_x86) = std::env::var("ProgramFiles(x86)") {
            candidates.push(
                PathBuf::from(&program_files_x86)
                    .join("VcXsrv")
                    .join("xauth.exe"),
            );
            candidates.push(
                PathBuf::from(&program_files_x86)
                    .join("Xming")
                    .join("xauth.exe"),
            );
        }
    }
    candidates
}

fn xauth_display_candidates(display: &str) -> Vec<String> {
    let display = display.trim();
    let mut candidates = Vec::new();
    if !display.is_empty() {
        candidates.push(display.to_string());
    }
    if let Some(display_number) = x11_display_number(display) {
        candidates.push(format!(":{display_number}"));
        candidates.push(format!("localhost:{display_number}"));
    }
    candidates.dedup();
    candidates
}

fn parse_xauth_cookie(stdout: &[u8]) -> Option<Vec<u8>> {
    let output = String::from_utf8_lossy(stdout);
    for line in output.lines() {
        let mut fields = line.split_whitespace();
        let Some(_display) = fields.next() else {
            continue;
        };
        let Some(protocol) = fields.next() else {
            continue;
        };
        let Some(cookie) = fields.next() else {
            continue;
        };
        if protocol.eq_ignore_ascii_case(X11_AUTH_PROTOCOL) {
            if let Ok(bytes) = hex::decode(cookie) {
                if !bytes.is_empty() {
                    return Some(bytes);
                }
            }
        }
    }
    None
}

async fn relay_x11_channel(
    channel: Channel<Msg>,
    mut local_x: Box<dyn AsyncReadWrite>,
    auth: LocalX11Auth,
    x11: Arc<X11ForwardingState>,
) -> Result<()> {
    let mut ssh_x = channel.into_stream();
    let setup = read_rewritten_x11_setup(&mut ssh_x, &x11, &auth)
        .await
        .context("rewrite X11 setup authentication cookie")?;
    local_x.write_all(&setup).await?;
    local_x.flush().await?;
    let (ssh_to_x, x_to_ssh) = tokio::io::copy_bidirectional(&mut ssh_x, local_x.as_mut()).await?;
    tracing::debug!(
        "[ssh:x11] X11 relay closed (ssh->x={} bytes, x->ssh={} bytes)",
        ssh_to_x,
        x_to_ssh
    );
    Ok(())
}

async fn read_rewritten_x11_setup<R: AsyncRead + Unpin>(
    ssh_x: &mut R,
    x11: &X11ForwardingState,
    auth: &LocalX11Auth,
) -> Result<Vec<u8>> {
    let mut header = [0u8; 12];
    ssh_x.read_exact(&mut header).await?;

    let byte_order = header[0];
    if byte_order != b'B' && byte_order != b'l' {
        return Err(anyhow!("invalid X11 byte order marker: {byte_order}"));
    }

    let auth_name_len = x11_u16(byte_order, header[6], header[7]) as usize;
    let auth_data_len = x11_u16(byte_order, header[8], header[9]) as usize;
    if auth_name_len > X11_MAX_AUTH_FIELD_LEN || auth_data_len > X11_MAX_AUTH_FIELD_LEN {
        return Err(anyhow!(
            "X11 auth fields too large: name={}, data={}",
            auth_name_len,
            auth_data_len
        ));
    }

    let auth_name_padded_len = pad4(auth_name_len);
    let auth_data_padded_len = pad4(auth_data_len);
    let mut rest = vec![0u8; auth_name_padded_len + auth_data_padded_len];
    ssh_x.read_exact(&mut rest).await?;

    let auth_name = &rest[..auth_name_len];
    let auth_data_offset = auth_name_padded_len;
    let auth_data_end = auth_data_offset + auth_data_len;
    let auth_data = &rest[auth_data_offset..auth_data_end];

    if auth_name != X11_AUTH_PROTOCOL.as_bytes() {
        return Err(anyhow!(
            "unsupported X11 auth protocol: {}",
            String::from_utf8_lossy(auth_name)
        ));
    }
    if auth_data != x11.fake_cookie {
        return Err(anyhow!(
            "X11 auth cookie did not match the forwarded fake cookie"
        ));
    }

    let real_name_len = auth.protocol.len();
    let real_data_len = auth.data.len();
    if real_name_len > X11_MAX_AUTH_FIELD_LEN || real_data_len > X11_MAX_AUTH_FIELD_LEN {
        return Err(anyhow!(
            "local X11 auth fields too large: name={}, data={}",
            real_name_len,
            real_data_len
        ));
    }

    put_x11_u16(byte_order, &mut header[6..8], real_name_len as u16);
    put_x11_u16(byte_order, &mut header[8..10], real_data_len as u16);

    let mut setup = Vec::with_capacity(header.len() + pad4(real_name_len) + pad4(real_data_len));
    setup.extend_from_slice(&header);
    setup.extend_from_slice(&auth.protocol);
    setup.resize(header.len() + pad4(real_name_len), 0);
    setup.extend_from_slice(&auth.data);
    setup.resize(header.len() + pad4(real_name_len) + pad4(real_data_len), 0);
    Ok(setup)
}

fn x11_u16(byte_order: u8, hi_or_lo: u8, lo_or_hi: u8) -> u16 {
    match byte_order {
        b'B' => u16::from_be_bytes([hi_or_lo, lo_or_hi]),
        b'l' => u16::from_le_bytes([hi_or_lo, lo_or_hi]),
        _ => 0,
    }
}

fn put_x11_u16(byte_order: u8, out: &mut [u8], value: u16) {
    let bytes = match byte_order {
        b'B' => value.to_be_bytes(),
        b'l' => value.to_le_bytes(),
        _ => [0, 0],
    };
    out.copy_from_slice(&bytes);
}

fn pad4(len: usize) -> usize {
    (len + 3) & !3
}

#[derive(Clone)]
struct ClientHandler {
    x11: Option<Arc<X11ForwardingState>>,
}

impl ClientHandler {
    fn new(x11: Option<Arc<X11ForwardingState>>) -> Self {
        Self { x11 }
    }
}

impl Handler for ClientHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &russh::keys::ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }

    fn server_channel_open_x11(
        &mut self,
        channel: Channel<Msg>,
        originator_address: &str,
        originator_port: u32,
        reply: client::ChannelOpenHandle,
        _session: &mut client::Session,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let x11 = self.x11.clone();
        let originator_address = originator_address.to_string();
        async move {
            let Some(x11) = x11 else {
                reply
                    .reject(ChannelOpenFailure::AdministrativelyProhibited)
                    .await;
                return Ok(());
            };

            match prepare_x11_relay(&x11).await {
                Ok((real_cookie, local_x)) => {
                    tracing::info!(
                        "[ssh:x11] accepting X11 channel from {}:{}",
                        originator_address,
                        originator_port
                    );
                    reply.accept().await;
                    tokio::spawn(async move {
                        if let Err(err) =
                            relay_x11_channel(channel, local_x, real_cookie, x11).await
                        {
                            tracing::warn!("[ssh:x11] X11 relay failed: {err:#}");
                        }
                    });
                }
                Err(err) => {
                    tracing::warn!(
                        "[ssh:x11] rejecting X11 channel from {}:{}: {err:#}",
                        originator_address,
                        originator_port
                    );
                    reply.reject(ChannelOpenFailure::ConnectFailed).await;
                }
            }
            Ok(())
        }
    }
}
