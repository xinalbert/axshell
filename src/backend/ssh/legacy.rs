use std::borrow::Cow;

use russh::{
    AlgorithmKind, Error as RusshError, Preferred, cipher,
    client::{self},
    kex,
    keys::ssh_key::Algorithm,
};

use crate::session::SshConnectionMode;

pub(crate) fn ssh_client_config(mode: SshConnectionMode) -> client::Config {
    let mut config = client::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(600)),
        keepalive_interval: Some(std::time::Duration::from_secs(3)),
        keepalive_max: 2,
        ..Default::default()
    };
    if mode == SshConnectionMode::Legacy {
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

pub(crate) fn negotiation_error_details(err: &anyhow::Error) -> Option<String> {
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
