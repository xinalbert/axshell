use std::sync::OnceLock;

use anyhow::Result;

use crate::{config::ConfigStore, session::Session};

pub(crate) trait ProxyStream:
    tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + Sync + 'static
{
}

impl<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + Sync + 'static> ProxyStream
    for T
{
}

#[derive(Debug, Clone)]
pub(crate) struct EnvProxy {
    pub(crate) proxy_type: String,
    pub(crate) host: String,
    pub(crate) port: Option<u16>,
    pub(crate) user: String,
    pub(crate) pass: String,
}

pub(crate) static ENV_PROXY: OnceLock<Option<EnvProxy>> = OnceLock::new();

pub(crate) async fn connect(session: &Session) -> Result<Box<dyn ProxyStream>> {
    let target_host = &session.host;
    let target_port = session.port;
    let (proxy_type, proxy_host, proxy_port, proxy_user, proxy_password) = resolved_proxy(session);

    if proxy_type != "none" && (proxy_host.is_empty() || proxy_port.is_none()) {
        let addr = format!("{}:{}", target_host, target_port);
        let stream = tokio::net::TcpStream::connect(&addr).await?;
        return Ok(Box::new(stream));
    }

    match proxy_type.as_str() {
        "socks5" | "socks5h" => {
            let proxy_port = proxy_port.unwrap_or(1080);
            let proxy_addr = format!("{}:{}", proxy_host, proxy_port);

            if !proxy_user.is_empty() {
                let stream = tokio_socks::tcp::Socks5Stream::connect_with_password(
                    proxy_addr.as_str(),
                    (target_host.as_str(), target_port),
                    &proxy_user,
                    &proxy_password,
                )
                .await
                .map_err(|e| anyhow::anyhow!("SOCKS5 proxy connection failed: {}", e))?;
                Ok(Box::new(stream))
            } else {
                let stream = tokio_socks::tcp::Socks5Stream::connect(
                    proxy_addr.as_str(),
                    (target_host.as_str(), target_port),
                )
                .await
                .map_err(|e| anyhow::anyhow!("SOCKS5 proxy connection failed: {}", e))?;
                Ok(Box::new(stream))
            }
        }
        "http" => {
            let proxy_port = proxy_port.unwrap_or(8080);
            let proxy_addr = format!("{}:{}", proxy_host, proxy_port);

            use tokio::io::AsyncWriteExt;
            let mut stream = tokio::net::TcpStream::connect(&proxy_addr)
                .await
                .map_err(|e| anyhow::anyhow!("HTTP proxy connection failed: {}", e))?;

            let mut request = format!(
                "CONNECT {}:{} HTTP/1.1\r\nHost: {}:{}\r\n",
                target_host, target_port, target_host, target_port
            );
            if !proxy_user.is_empty() {
                use base64::Engine as _;
                let auth = format!("{}:{}", proxy_user, proxy_password);
                let encoded = base64::engine::general_purpose::STANDARD.encode(auth);
                request.push_str(&format!("Proxy-Authorization: Basic {}\r\n", encoded));
            }
            request.push_str("\r\n");

            stream.write_all(request.as_bytes()).await?;

            let mut response = [0u8; 1024];
            let n = tokio::io::AsyncReadExt::read(&mut stream, &mut response).await?;
            let resp_str = String::from_utf8_lossy(&response[..n]);
            if !resp_str.contains("200") && !resp_str.contains("established") {
                return Err(anyhow::anyhow!("HTTP proxy CONNECT failed: {}", resp_str));
            }

            Ok(Box::new(stream))
        }
        _ => {
            let addr = format!("{}:{}", target_host, target_port);
            let stream = tokio::net::TcpStream::connect(&addr).await?;
            Ok(Box::new(stream))
        }
    }
}

pub(crate) fn active(session: &Session) -> Option<(String, String, Option<u16>)> {
    let (proxy_type, proxy_host, proxy_port, _, _) = resolved_proxy(session);
    if proxy_type != "none" && !proxy_host.is_empty() && proxy_port.is_some() {
        Some((proxy_type, proxy_host, proxy_port))
    } else {
        None
    }
}

fn resolved_proxy(session: &Session) -> (String, String, Option<u16>, String, String) {
    let config = ConfigStore::load().unwrap_or_else(|_| ConfigStore::in_memory());
    if !session.proxy_type.is_empty() && session.proxy_type != "none" {
        return (
            session.proxy_type.clone(),
            session.proxy_host.clone(),
            session.proxy_port,
            session.proxy_user.clone(),
            session.proxy_password.clone(),
        );
    }

    if config.read_env_proxy()
        && let Some(env_proxy) = ENV_PROXY.get().and_then(|proxy| proxy.as_ref())
    {
        return (
            env_proxy.proxy_type.clone(),
            env_proxy.host.clone(),
            env_proxy.port,
            env_proxy.user.clone(),
            env_proxy.pass.clone(),
        );
    }

    if config.use_proxy() {
        return (
            config.global_proxy_type().to_string(),
            config.global_proxy_host().to_string(),
            config.global_proxy_port(),
            config.global_proxy_user().to_string(),
            config.global_proxy_password().to_string(),
        );
    }

    (
        "none".to_string(),
        String::new(),
        None,
        String::new(),
        String::new(),
    )
}
