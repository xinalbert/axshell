use chrono::{DateTime, TimeZone, Utc};

pub(super) fn base_name(path: &str) -> String {
    let sep = |c: char| c == '/' || c == '\\';
    path.trim_end_matches(sep)
        .rsplit(sep)
        .next()
        .unwrap_or(path)
        .to_string()
}

pub(crate) fn parent_dir(path: &str) -> Option<String> {
    if path == "/" || path.is_empty() {
        return None;
    }
    let trimmed = path.trim_end_matches('/');
    if let Some(idx) = trimmed.rfind('/') {
        if idx == 0 {
            Some("/".to_string())
        } else {
            Some(trimmed[..idx].to_string())
        }
    } else {
        Some("/".to_string())
    }
}

pub(crate) fn join_remote(parent: &str, child: &str) -> String {
    if parent == "/" {
        format!("/{child}")
    } else {
        format!("{}/{}", parent.trim_end_matches('/'), child)
    }
}

pub(crate) fn normalize_remote_path(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return "/".to_string();
    }

    let anchored = trimmed.starts_with('/');
    let mut parts: Vec<&str> = Vec::new();
    for part in trimmed.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }

    if anchored {
        if parts.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", parts.join("/"))
        }
    } else if parts.is_empty() {
        ".".to_string()
    } else {
        parts.join("/")
    }
}

pub(crate) fn resolve_remote_path(current_dir: &str, path: &str, home_dir: &str) -> String {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return normalize_remote_path(current_dir);
    }

    if trimmed == "~" {
        return normalize_remote_path(home_dir);
    }
    if let Some(rest) = trimmed.strip_prefix("~/") {
        return normalize_remote_path(&join_remote(home_dir, rest));
    }

    if trimmed.starts_with('/') {
        return normalize_remote_path(trimmed);
    }

    normalize_remote_path(&join_remote(current_dir, trimmed))
}

#[allow(dead_code)]
pub(super) fn strip_archive_suffix(name: &str) -> &str {
    for suffix in [".tar.gz", ".tgz", ".zip", ".tar"] {
        if let Some(stripped) = name.strip_suffix(suffix) {
            return stripped;
        }
    }
    name
}

pub(super) fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

pub fn format_mtime(ts: u32) -> String {
    let dt: DateTime<Utc> = Utc
        .timestamp_opt(ts as i64, 0)
        .single()
        .unwrap_or_else(Utc::now);
    dt.format("%Y-%m-%d %H:%M").to_string()
}

pub(super) fn remote_parent(path: &str) -> String {
    if path == "/" {
        "/".to_string()
    } else {
        path.rsplit_once('/')
            .map(|(parent, _)| {
                if parent.is_empty() {
                    "/".to_string()
                } else {
                    parent.to_string()
                }
            })
            .unwrap_or_else(|| "/".to_string())
    }
}

pub(super) fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

#[cfg(test)]
mod tests {
    use super::{normalize_remote_path, resolve_remote_path};

    #[test]
    fn normalize_remote_path_resolves_dot_segments() {
        assert_eq!(
            normalize_remote_path("/srv/app/./logs/../tmp"),
            "/srv/app/tmp"
        );
        assert_eq!(normalize_remote_path("/../../etc"), "/etc");
    }

    #[test]
    fn resolve_remote_path_uses_current_dir_for_relative_paths() {
        assert_eq!(
            resolve_remote_path("/srv/app/current", "../logs/output.txt", "/home/test"),
            "/srv/app/logs/output.txt"
        );
    }

    #[test]
    fn resolve_remote_path_preserves_absolute_paths_and_home() {
        assert_eq!(
            resolve_remote_path("/srv/app", "/var/log/syslog", "/home/test"),
            "/var/log/syslog"
        );
        assert_eq!(
            resolve_remote_path("/srv/app", "~/notes/todo.txt", "/home/test"),
            "/home/test/notes/todo.txt"
        );
    }
}
