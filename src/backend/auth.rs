use std::{path::Path, sync::Arc};

use anyhow::{Result, anyhow};
use directories::BaseDirs;
use russh::keys::{
    HashAlg, PrivateKey, decode_secret_key, key::PrivateKeyWithHashAlg, load_secret_key,
};

use crate::session::Session;

pub(crate) fn load_session_private_key(session: &Session) -> Result<PrivateKey> {
    let inline_key = normalize_inline_private_key(&session.private_key_inline);
    let key_path = expand_key_path(session.private_key_path.trim());
    let passphrase = session.passphrase.trim();
    let passphrase = (!passphrase.is_empty()).then_some(passphrase);
    let has_inline = !inline_key.is_empty();
    let has_path = key_path.is_some();

    if !has_inline && !has_path {
        return Err(anyhow!("private key content or path is required"));
    }

    let mut errors = Vec::new();

    if has_inline {
        match decode_secret_key(&inline_key, passphrase) {
            Ok(key) => return Ok(key),
            Err(err) => errors.push(format!("decode private key content: {err}")),
        }
    }

    if let Some(path) = key_path {
        match load_secret_key(path.as_path(), passphrase) {
            Ok(key) => return Ok(key),
            Err(err) => errors.push(format!("load key {}: {err}", path.display())),
        }
    }

    Err(anyhow!(errors.join("; ")))
}

pub(crate) fn private_keys_with_algs(keypair: PrivateKey) -> Result<Vec<PrivateKeyWithHashAlg>> {
    let mut algs = Vec::new();
    let key_arc = Arc::new(keypair);

    if key_arc.algorithm().is_rsa() {
        algs.push(PrivateKeyWithHashAlg::new(
            key_arc.clone(),
            Some(HashAlg::Sha512),
        ));
        algs.push(PrivateKeyWithHashAlg::new(
            key_arc.clone(),
            Some(HashAlg::Sha256),
        ));
        algs.push(PrivateKeyWithHashAlg::new(key_arc.clone(), None));
    } else {
        algs.push(PrivateKeyWithHashAlg::new(key_arc.clone(), None));
    }

    if algs.is_empty() {
        return Err(anyhow!(
            "Failed to construct PrivateKeyWithHashAlg for any supported hash algorithm"
        ));
    }

    Ok(algs)
}

fn normalize_inline_private_key(value: &str) -> String {
    let mut normalized = value
        .trim()
        .replace("\\r\\n", "\n")
        .replace("\\n", "\n")
        .replace("\r\n", "\n");
    if !normalized.ends_with('\n') {
        normalized.push('\n');
    }
    normalized
}

fn expand_key_path(value: &str) -> Option<std::path::PathBuf> {
    if value.is_empty() {
        return None;
    }
    if value == "~" {
        return BaseDirs::new().map(|dirs| dirs.home_dir().to_path_buf());
    }
    if let Some(rest) = value.strip_prefix("~/") {
        return BaseDirs::new().map(|dirs| dirs.home_dir().join(rest));
    }
    Some(Path::new(value).to_path_buf())
}
