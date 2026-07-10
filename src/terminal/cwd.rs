pub(super) fn extract_shell_working_directory(bytes: &[u8]) -> (Option<String>, Vec<u8>) {
    const OSC_PREFIX: &[u8] = b"\x1b]";
    let mut cursor = 0;
    let mut found = None;
    let mut pending = Vec::new();

    while let Some(relative_start) = find_bytes(&bytes[cursor..], OSC_PREFIX) {
        let start = cursor + relative_start;
        let content_start = start + OSC_PREFIX.len();
        let Some((end, terminator_len)) = find_osc_terminator(&bytes[content_start..]) else {
            pending.extend_from_slice(&bytes[start..]);
            break;
        };
        let end = content_start + end;
        if let Some(path) = parse_working_directory_osc(&bytes[content_start..end]) {
            found = Some(path);
        }
        cursor = end + terminator_len;
    }

    const MAX_PENDING_OSC_BYTES: usize = 4096;
    if pending.len() > MAX_PENDING_OSC_BYTES {
        pending.clear();
    }

    (found, pending)
}

fn parse_working_directory_osc(content: &[u8]) -> Option<String> {
    let content = std::str::from_utf8(content).ok()?;

    if let Some(rest) = content.strip_prefix("633;P;") {
        for property in rest.split(';') {
            if let Some(cwd) = property.strip_prefix("Cwd=") {
                return normalize_shell_working_directory(cwd);
            }
        }
    }

    if let Some(current_dir) = content.strip_prefix("1337;CurrentDir=") {
        return normalize_shell_working_directory(current_dir);
    }

    if let Some(uri) = content.strip_prefix("7;") {
        return normalize_shell_working_directory(uri);
    }

    None
}

fn normalize_shell_working_directory(value: &str) -> Option<String> {
    let decoded = percent_decode(value);
    let path = if let Some(uri_path) = decoded.strip_prefix("file://") {
        match uri_path.find('/') {
            Some(index) => &uri_path[index..],
            None => "",
        }
    } else if let Some(uri_path) = decoded.strip_prefix("kitty-shell-cwd://") {
        match uri_path.find('/') {
            Some(index) => &uri_path[index..],
            None => "",
        }
    } else {
        decoded.as_str()
    };
    let path = path.trim();
    if path.starts_with('/') {
        Some(path.to_string())
    } else {
        None
    }
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%'
            && index + 2 < bytes.len()
            && let (Some(high), Some(low)) =
                (hex_value(bytes[index + 1]), hex_value(bytes[index + 2]))
        {
            decoded.push(high << 4 | low);
            index += 3;
            continue;
        }
        decoded.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&decoded).to_string()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn find_osc_terminator(bytes: &[u8]) -> Option<(usize, usize)> {
    let bel = bytes.iter().position(|byte| *byte == b'\x07');
    let st = find_bytes(bytes, b"\x1b\\");
    match (bel, st) {
        (Some(bel), Some(st)) if bel < st => Some((bel, 1)),
        (Some(_), Some(st)) => Some((st, 2)),
        (Some(bel), None) => Some((bel, 1)),
        (None, Some(st)) => Some((st, 2)),
        (None, None) => None,
    }
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::extract_shell_working_directory;

    #[test]
    fn captures_vscode_cwd_with_bel() {
        let (path, pending) =
            extract_shell_working_directory(b"prompt\x1b]633;P;Cwd=/srv/app\x07$ ");

        assert_eq!(path.as_deref(), Some("/srv/app"));
        assert!(pending.is_empty());
    }

    #[test]
    fn captures_vscode_cwd_with_st() {
        let (path, pending) =
            extract_shell_working_directory(b"\x1b]633;P;Some=1;Cwd=/opt/project\x1b\\");

        assert_eq!(path.as_deref(), Some("/opt/project"));
        assert!(pending.is_empty());
    }

    #[test]
    fn captures_iterm2_current_dir() {
        let (path, pending) =
            extract_shell_working_directory(b"\x1b]1337;CurrentDir=/Users/test/work\x07");

        assert_eq!(path.as_deref(), Some("/Users/test/work"));
        assert!(pending.is_empty());
    }

    #[test]
    fn captures_osc7_file_uri_with_percent_decoding() {
        let (path, pending) =
            extract_shell_working_directory(b"\x1b]7;file://host/home/test/My%20Project\x07");

        assert_eq!(path.as_deref(), Some("/home/test/My Project"));
        assert!(pending.is_empty());
    }

    #[test]
    fn captures_kitty_cwd_uri() {
        let (path, pending) = extract_shell_working_directory(
            b"\x1b]7;kitty-shell-cwd://host/home/test/Kitty%20Project\x07",
        );

        assert_eq!(path.as_deref(), Some("/home/test/Kitty Project"));
        assert!(pending.is_empty());
    }

    #[test]
    fn carries_incomplete_osc_until_terminated() {
        let (path, pending) = extract_shell_working_directory(b"\x1b]633;P;Cwd=/tmp");

        assert_eq!(path, None);
        assert_eq!(pending, b"\x1b]633;P;Cwd=/tmp");

        let mut combined = pending;
        combined.extend_from_slice(b"\x07");
        let (path, pending) = extract_shell_working_directory(&combined);

        assert_eq!(path.as_deref(), Some("/tmp"));
        assert!(pending.is_empty());
    }
}
