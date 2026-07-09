pub(crate) const DEFAULT_COLS: u16 = 100;
pub(crate) const DEFAULT_ROWS: u16 = 30;
pub(crate) const SIDEBAR_WIDTH: f32 = 306.0;
pub(crate) const COLLAPSED_SIDEBAR_WIDTH: f32 = 52.0;

#[allow(dead_code)]
pub(crate) const TAB_BAR_HEIGHT: f32 = 52.0;
#[allow(dead_code)]
pub(crate) const TERMINAL_PADDING_X: f32 = 32.0;
#[allow(dead_code)]
pub(crate) const TERMINAL_PADDING_Y: f32 = 32.0;

pub(crate) const TERMINAL_KEY_CONTEXT: &str = "AxShellTerminal";
pub(crate) const REPOSITORY_URL: &str = "https://github.com/xinalbert/axshell";
pub(crate) const ISSUES_URL: &str = "https://github.com/xinalbert/axshell/issues";

pub(crate) fn public_version_label() -> String {
    format_public_version(env!("CARGO_PKG_VERSION"))
}

fn format_public_version(version: &str) -> String {
    let version = version.split('+').next().unwrap_or(version);
    let (core, suffix) = version
        .split_once('-')
        .map_or((version, None), |(core, suffix)| (core, Some(suffix)));

    let mut parts = core.split('.');
    let (Some(year), Some(month), Some(day), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return version.to_string();
    };

    let (Ok(year), Ok(month), Ok(day)) = (
        year.parse::<u32>(),
        month.parse::<u32>(),
        day.parse::<u32>(),
    ) else {
        return version.to_string();
    };

    let mut public = format!("{year:04}.{month:02}.{day:02}");
    if let Some(suffix) = suffix.filter(|suffix| !suffix.is_empty()) {
        public.push('.');
        public.push_str(suffix);
    }
    public
}
