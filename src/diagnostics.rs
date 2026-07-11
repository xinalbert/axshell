pub(crate) fn mask_value(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return "***".to_string();
    }

    let chars: Vec<char> = trimmed.chars().collect();
    if chars.len() <= 2 {
        return "*".to_string();
    }
    if chars.len() <= 4 {
        return format!("{}*", chars[0]);
    }

    let prefix: String = chars.iter().take(2).collect();
    let suffix: String = chars.iter().skip(chars.len().saturating_sub(2)).collect();
    format!("{prefix}*{suffix}")
}

pub(crate) fn mask_host(host: &str) -> String {
    let trimmed = host.trim();
    let ipv4_parts: Vec<&str> = trimmed.split('.').collect();
    if ipv4_parts.len() == 4
        && ipv4_parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|ch| ch.is_ascii_digit()))
    {
        return format!("{}.*.*.{}", ipv4_parts[0], ipv4_parts[3]);
    }

    let ipv6_parts: Vec<&str> = trimmed.split(':').filter(|part| !part.is_empty()).collect();
    if trimmed.contains(':') && ipv6_parts.len() >= 2 {
        return format!(
            "{}:****:{}",
            ipv6_parts.first().unwrap_or(&""),
            ipv6_parts.last().unwrap_or(&"")
        );
    }

    mask_value(trimmed)
}

pub(crate) fn mask_path(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return "***".to_string();
    }

    let normalized = trimmed.replace('\\', "/");
    let Some(name) = normalized.rsplit('/').find(|part| !part.is_empty()) else {
        return "/".to_string();
    };
    format!(".../{}", mask_value(name))
}

pub(crate) fn sanitize_error(message: &str) -> String {
    const MAX_ERROR_CHARS: usize = 512;

    let sanitized = message
        .split_whitespace()
        .map(|part| {
            if part.starts_with('/')
                || part.starts_with("~/")
                || part.contains('\\')
                || part.as_bytes().get(1) == Some(&b':')
            {
                mask_path(part)
            } else {
                part.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    if sanitized.chars().count() <= MAX_ERROR_CHARS {
        return sanitized;
    }

    let mut truncated = sanitized
        .chars()
        .take(MAX_ERROR_CHARS.saturating_sub(3))
        .collect::<String>();
    truncated.push_str("...");
    truncated
}

pub(crate) fn sanitize_error_with_values(message: &str, values: &[&str]) -> String {
    let mut values = values
        .iter()
        .copied()
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    values.sort_unstable_by_key(|value| std::cmp::Reverse(value.len()));

    let mut sanitized = message.to_string();
    for value in values {
        sanitized = sanitized.replace(value, "***");
    }
    sanitize_error(&sanitized)
}

#[cfg(test)]
mod tests {
    use super::{mask_host, mask_path, mask_value, sanitize_error, sanitize_error_with_values};

    #[test]
    fn masks_user_values_and_hosts() {
        assert_eq!(mask_value("alice"), "al*ce");
        assert_eq!(mask_host("192.168.1.42"), "192.*.*.42");
        assert_eq!(mask_host("server.example.com"), "se*om");
        assert_eq!(mask_host("2001:db8::1"), "2001:****:1");
    }

    #[test]
    fn masks_only_the_path_tail() {
        assert_eq!(mask_path("/Users/alice/secrets.txt"), ".../se*xt");
        assert_eq!(mask_path(r"C:\Users\alice\keys"), ".../k*");
        assert_eq!(mask_path("/"), "/");
        assert_eq!(mask_path(""), "***");
    }

    #[test]
    fn sanitizes_paths_and_multiline_errors() {
        assert_eq!(
            sanitize_error("failed to read /Users/alice/secrets.txt\npermission denied"),
            "failed to read .../se*xt permission denied"
        );
        assert_eq!(
            sanitize_error(r"failed to open C:\Users\alice\keys"),
            "failed to open .../k*"
        );
    }

    #[test]
    fn removes_known_sensitive_values_from_errors() {
        assert_eq!(
            sanitize_error_with_values(
                "authentication failed for alice@server.example.com using /Users/alice/id_ed25519",
                &["alice", "server.example.com", "/Users/alice/id_ed25519"]
            ),
            "authentication failed for ***@*** using ***"
        );
    }
}
