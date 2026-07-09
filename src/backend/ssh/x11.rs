use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::{Context, Result, anyhow};
use rand::RngCore;
use russh::{Channel, ChannelOpenFailure, client};
#[cfg(unix)]
use tokio::net::UnixStream;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpStream,
    time::{Duration, sleep},
};

use crate::session::config::ConfigStore;

const X11_AUTH_PROTOCOL: &str = "MIT-MAGIC-COOKIE-1";
const X11_REMOTE_DISPLAY: &str = "localhost:10.0";
const X11_DEFAULT_SCREEN: u32 = 0;
const X11_MAX_AUTH_FIELD_LEN: usize = 4096;

trait AsyncReadWrite: AsyncRead + AsyncWrite + Unpin + Send {}

impl<T> AsyncReadWrite for T where T: AsyncRead + AsyncWrite + Unpin + Send {}

pub(super) struct X11ForwardingState {
    fake_cookie: [u8; 16],
    pub(super) fake_cookie_hex: String,
    local_display: Mutex<String>,
    pub(super) remote_display: String,
    pub(super) screen_number: u32,
    launch_local_x_server: bool,
    local_x_server_app_path: String,
}

impl X11ForwardingState {
    pub(super) fn from_config(config: &ConfigStore) -> Option<Arc<Self>> {
        if !config.x11_forwarding_enabled() {
            return None;
        }

        let mut fake_cookie = [0u8; 16];
        rand::rngs::OsRng.fill_bytes(&mut fake_cookie);
        let local_display = crate::session::config::resolve_local_x_display(
            config.local_x_server_app_path(),
            config.x11_launch_local_x_server(),
        );

        Some(Arc::new(Self {
            fake_cookie,
            fake_cookie_hex: hex::encode(fake_cookie),
            local_display: Mutex::new(local_display),
            remote_display: X11_REMOTE_DISPLAY.to_string(),
            screen_number: X11_DEFAULT_SCREEN,
            launch_local_x_server: config.x11_launch_local_x_server(),
            local_x_server_app_path: config.local_x_server_app_path().to_string(),
        }))
    }

    fn local_display(&self) -> String {
        self.local_display
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    fn set_local_display(&self, display: String) {
        *self
            .local_display
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = display;
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

pub(super) async fn handle_x11_channel(
    channel: Channel<client::Msg>,
    originator_address: String,
    originator_port: u32,
    reply: client::ChannelOpenHandle,
    x11: Arc<X11ForwardingState>,
) -> Result<()> {
    match prepare_x11_relay(&x11).await {
        Ok((real_cookie, local_x)) => {
            tracing::info!(
                "[ssh:x11] accepting X11 channel from {}:{}",
                originator_address,
                originator_port
            );
            reply.accept().await;
            tokio::spawn(async move {
                if let Err(err) = relay_x11_channel(channel, local_x, real_cookie, x11).await {
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

async fn prepare_x11_relay(
    x11: &X11ForwardingState,
) -> Result<(LocalX11Auth, Box<dyn AsyncReadWrite>)> {
    let mut local_display = x11.local_display();
    if x11.launch_local_x_server {
        match crate::app::startup::launch_local_x_server_app(&x11.local_x_server_app_path) {
            Ok(display) => {
                if display != local_display {
                    x11.set_local_display(display.clone());
                    local_display = display;
                }
                sleep(Duration::from_millis(700)).await;
            }
            Err(err) => tracing::warn!("[ssh:x11] failed to launch configured X server: {err:#}"),
        }
    }

    let auth = match load_local_x11_cookie(&local_display) {
        Ok(cookie) => LocalX11Auth {
            protocol: X11_AUTH_PROTOCOL.as_bytes().to_vec(),
            data: cookie,
        },
        Err(err) => {
            tracing::warn!(
                "[ssh:x11] no local X11 MIT cookie found for {}; falling back to no-auth setup: {err:#}",
                local_display
            );
            LocalX11Auth {
                protocol: Vec::new(),
                data: Vec::new(),
            }
        }
    };
    let local_x = connect_local_x11(&local_display)
        .await
        .with_context(|| format!("connect local X server at {}", local_display))?;
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
    channel: Channel<client::Msg>,
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
