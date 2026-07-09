use anyhow::{Context as _, Result, anyhow};
use gpui::{App, Context, SharedString, Window, px};
use gpui_component::{
    ActiveTheme as _, Theme, ThemeConfig, ThemeMode, ThemeRegistry, ThemeSet, try_parse_color,
};
use serde_json::{Map as JsonMap, Value as JsonValue};

use crate::{
    AxShell,
    session::config::{ConfigStore, CustomThemeModeConfig},
};

pub(crate) const EMBEDDED_THEME_JSONS: &[&str] = &[
    include_str!("../../assets/themes/matrix.json"),
    include_str!("../../assets/themes/tokyonight.json"),
    include_str!("../../assets/themes/gruvbox.json"),
    include_str!("../../assets/themes/solarized.json"),
    include_str!("../../assets/themes/phygerr.json"),
];

use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};

pub(crate) static USING_SYSTEM_MAPLE: AtomicBool = AtomicBool::new(false);

const CUSTOM_THEME_NAME_INPUT_KEY: &str = "custom_theme.theme_name";
const CUSTOM_THEME_FILE_PREFIX: &str = "custom-";
const CUSTOM_LIGHT_SUFFIX: &str = "[Custom Light]";
const CUSTOM_DARK_SUFFIX: &str = "[Custom Dark]";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CustomThemeFieldDomain {
    ThemeColor,
    HighlightColor,
    Brightness,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CustomThemeFieldSpec {
    pub key: &'static str,
    pub label: &'static str,
    pub placeholder: &'static str,
    pub domain: CustomThemeFieldDomain,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CustomThemeSectionSpec {
    pub title: &'static str,
    pub fields: &'static [CustomThemeFieldSpec],
}

pub(crate) const CUSTOM_THEME_CORE_FIELDS: &[CustomThemeFieldSpec] = &[
    CustomThemeFieldSpec {
        key: "background",
        label: "Background",
        placeholder: "#111827",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "foreground",
        label: "Foreground",
        placeholder: "#E5E7EB",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "border",
        label: "Border",
        placeholder: "#374151",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "primary.background",
        label: "Primary Background",
        placeholder: "#4F8CFF",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "primary.foreground",
        label: "Primary Foreground",
        placeholder: "#FFFFFF",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "secondary.background",
        label: "Secondary Background",
        placeholder: "#1F2937",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "secondary.foreground",
        label: "Secondary Foreground",
        placeholder: "#E5E7EB",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "accent.background",
        label: "Accent Background",
        placeholder: "#1D4ED8",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "accent.foreground",
        label: "Accent Foreground",
        placeholder: "#FFFFFF",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "selection.background",
        label: "Selection Background",
        placeholder: "#2563EB66",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "ring",
        label: "Focus Ring",
        placeholder: "#60A5FA",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "font_brightness",
        label: "Font Brightness",
        placeholder: "1.00",
        domain: CustomThemeFieldDomain::Brightness,
    },
];

pub(crate) const CUSTOM_THEME_SURFACE_FIELDS: &[CustomThemeFieldSpec] = &[
    CustomThemeFieldSpec {
        key: "popover.background",
        label: "Popover Background",
        placeholder: "#111827",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "popover.foreground",
        label: "Popover Foreground",
        placeholder: "#E5E7EB",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "sidebar.background",
        label: "Sidebar Background",
        placeholder: "#0F172A",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "sidebar.foreground",
        label: "Sidebar Foreground",
        placeholder: "#E5E7EB",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "sidebar.primary.background",
        label: "Sidebar Primary Background",
        placeholder: "#4F8CFF",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "sidebar.primary.foreground",
        label: "Sidebar Primary Foreground",
        placeholder: "#FFFFFF",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "tab.active.background",
        label: "Active Tab Background",
        placeholder: "#111827",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "tab.active.foreground",
        label: "Active Tab Foreground",
        placeholder: "#F9FAFB",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "tab.foreground",
        label: "Tab Foreground",
        placeholder: "#CBD5E1",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "table.head.background",
        label: "Table Head Background",
        placeholder: "#111827",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "table.head.foreground",
        label: "Table Head Foreground",
        placeholder: "#CBD5E1",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
];

pub(crate) const CUSTOM_THEME_SEMANTIC_FIELDS: &[CustomThemeFieldSpec] = &[
    CustomThemeFieldSpec {
        key: "danger.background",
        label: "Danger Background",
        placeholder: "#EF4444",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "danger.foreground",
        label: "Danger Foreground",
        placeholder: "#FFFFFF",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "info.background",
        label: "Info Background",
        placeholder: "#06B6D4",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "success.background",
        label: "Success Background",
        placeholder: "#22C55E",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "warning.background",
        label: "Warning Background",
        placeholder: "#F59E0B",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "base.red",
        label: "Base Red",
        placeholder: "#EF4444",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "base.green",
        label: "Base Green",
        placeholder: "#22C55E",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "base.blue",
        label: "Base Blue",
        placeholder: "#3B82F6",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "base.yellow",
        label: "Base Yellow",
        placeholder: "#F59E0B",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "base.magenta",
        label: "Base Magenta",
        placeholder: "#A855F7",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "base.cyan",
        label: "Base Cyan",
        placeholder: "#06B6D4",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
];

pub(crate) const CUSTOM_THEME_EDITOR_FIELDS: &[CustomThemeFieldSpec] = &[
    CustomThemeFieldSpec {
        key: "editor.background",
        label: "Editor Background",
        placeholder: "#111827",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "editor.foreground",
        label: "Editor Foreground",
        placeholder: "#E5E7EB",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "editor.active_line.background",
        label: "Active Line Background",
        placeholder: "#1F2937",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "editor.line_number",
        label: "Line Number",
        placeholder: "#64748B",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "editor.active_line_number",
        label: "Active Line Number",
        placeholder: "#F8FAFC",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "syntax.keyword.color",
        label: "Syntax Keyword",
        placeholder: "#F472B6",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "syntax.string.color",
        label: "Syntax String",
        placeholder: "#86EFAC",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "syntax.function.color",
        label: "Syntax Function",
        placeholder: "#60A5FA",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "syntax.type.color",
        label: "Syntax Type",
        placeholder: "#C084FC",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "syntax.comment.color",
        label: "Syntax Comment",
        placeholder: "#64748B",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
];

pub(crate) const CUSTOM_THEME_SECTION_SPECS: &[CustomThemeSectionSpec] = &[
    CustomThemeSectionSpec {
        title: "Core",
        fields: CUSTOM_THEME_CORE_FIELDS,
    },
    CustomThemeSectionSpec {
        title: "Surfaces",
        fields: CUSTOM_THEME_SURFACE_FIELDS,
    },
    CustomThemeSectionSpec {
        title: "Semantic & Base Palette",
        fields: CUSTOM_THEME_SEMANTIC_FIELDS,
    },
    CustomThemeSectionSpec {
        title: "Editor & Syntax",
        fields: CUSTOM_THEME_EDITOR_FIELDS,
    },
];

pub(crate) fn custom_theme_name_input_key() -> &'static str {
    CUSTOM_THEME_NAME_INPUT_KEY
}

pub(crate) fn custom_theme_modes() -> [ThemeMode; 2] {
    [ThemeMode::Light, ThemeMode::Dark]
}

pub(crate) fn custom_theme_input_key(mode: ThemeMode, key: &str) -> String {
    format!(
        "custom_theme.{}.{}",
        if mode.is_dark() { "dark" } else { "light" },
        key
    )
}

pub(crate) fn custom_theme_registry_name(theme_name: &str, mode: ThemeMode) -> String {
    let theme_name = normalized_custom_theme_name(theme_name);
    if mode.is_dark() {
        format!("{theme_name} {CUSTOM_DARK_SUFFIX}")
    } else {
        format!("{theme_name} {CUSTOM_LIGHT_SUFFIX}")
    }
}

pub(crate) fn load_fonts(cx: &mut App) -> Result<()> {
    let has_system_maple = cx
        .text_system()
        .all_font_names()
        .contains(&"Maple Mono NF CN".to_string());
    if has_system_maple {
        USING_SYSTEM_MAPLE.store(true, Ordering::Relaxed);
    } else {
        let regular = Cow::Borrowed(
            include_bytes!("../../assets/fonts/MapleMono-NF-CN-Regular.ttf").as_slice(),
        );
        let bold =
            Cow::Borrowed(include_bytes!("../../assets/fonts/MapleMono-NF-CN-Bold.ttf").as_slice());
        cx.text_system()
            .add_fonts(vec![regular, bold])
            .context("load Maple Mono NF CN fonts")?;
    }
    set_theme_font_names(cx.global_mut::<Theme>(), ".SystemUIFont");
    Ok(())
}

pub(crate) fn load_embedded_themes(cx: &mut App) {
    let registry = ThemeRegistry::global_mut(cx);
    for theme_json in EMBEDDED_THEME_JSONS {
        if let Err(err) = registry.load_themes_from_str(theme_json) {
            tracing::warn!("failed to load embedded theme: {err:#}");
        }
    }
}

pub(crate) fn load_user_themes(cx: &mut App) {
    let Ok(themes_dir) = ConfigStore::theme_dir_path() else {
        tracing::warn!("failed to resolve user theme dir");
        return;
    };

    if let Err(err) = fs::create_dir_all(&themes_dir) {
        tracing::warn!(
            "failed to create user theme dir {}: {err:#}",
            themes_dir.display()
        );
        return;
    }

    let registry = ThemeRegistry::global_mut(cx);
    if let Ok(entries) = fs::read_dir(&themes_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }
            match fs::read_to_string(&path) {
                Ok(content) => {
                    if let Err(err) = registry.load_themes_from_str(&content) {
                        tracing::warn!("failed to load user theme {}: {err:#}", path.display());
                    }
                }
                Err(err) => {
                    tracing::warn!("failed to read user theme {}: {err:#}", path.display());
                }
            }
        }
    }

    if let Err(err) = ThemeRegistry::watch_dir(themes_dir, cx, |_| {}) {
        tracing::warn!("failed to watch user theme dir: {err:#}");
    }
}

pub(crate) fn set_theme_font_names(theme: &mut Theme, ui_font_family: &str) {
    theme.font_family = ui_font_family.into();
    theme.mono_font_family = ui_font_family.into();
}

fn normalized_custom_theme_name(theme_name: &str) -> String {
    let theme_name = theme_name.trim();
    if theme_name.is_empty() {
        "Custom Theme".to_string()
    } else {
        theme_name.to_string()
    }
}

fn custom_theme_file_path(theme_dir: &Path, theme_name: &str) -> PathBuf {
    let mut slug = String::new();
    for ch in normalized_custom_theme_name(theme_name).chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_matches('-');
    let slug = if slug.is_empty() {
        "custom-theme"
    } else {
        slug
    };
    theme_dir.join(format!("{CUSTOM_THEME_FILE_PREFIX}{slug}.json"))
}

fn custom_theme_field_specs() -> impl Iterator<Item = &'static CustomThemeFieldSpec> {
    CUSTOM_THEME_SECTION_SPECS
        .iter()
        .flat_map(|section| section.fields.iter())
}

fn find_custom_theme_field(key: &str) -> Option<&'static CustomThemeFieldSpec> {
    custom_theme_field_specs().find(|field| field.key == key)
}

fn is_highlight_status_key(key: &str) -> bool {
    matches!(key, "error" | "warning" | "info" | "success" | "hint")
        || key.starts_with("error.")
        || key.starts_with("warning.")
        || key.starts_with("info.")
        || key.starts_with("success.")
        || key.starts_with("hint.")
}

fn set_syntax_override(
    highlight_object: &mut JsonMap<String, JsonValue>,
    key: &str,
    value: &str,
) -> Result<()> {
    let Some(rest) = key.strip_prefix("syntax.") else {
        return Err(anyhow!("invalid syntax override key: {key}"));
    };
    let Some(token) = rest.strip_suffix(".color") else {
        return Err(anyhow!("unsupported syntax override key: {key}"));
    };

    let syntax = highlight_object
        .entry("syntax".to_string())
        .or_insert_with(|| JsonValue::Object(JsonMap::new()));
    let syntax = syntax
        .as_object_mut()
        .ok_or_else(|| anyhow!("syntax highlight section is not an object"))?;
    let style = syntax
        .entry(token.to_string())
        .or_insert_with(|| JsonValue::Object(JsonMap::new()));
    let style = style
        .as_object_mut()
        .ok_or_else(|| anyhow!("syntax style entry is not an object"))?;
    style.insert("color".to_string(), JsonValue::String(value.to_string()));
    Ok(())
}

fn build_custom_theme_config(
    base_theme: &ThemeConfig,
    mode_config: &CustomThemeModeConfig,
    generated_name: &str,
    mode: ThemeMode,
) -> Result<ThemeConfig> {
    let mut value = serde_json::to_value(base_theme.clone()).context("serialize base theme")?;
    let object = value
        .as_object_mut()
        .ok_or_else(|| anyhow!("serialized theme config is not an object"))?;

    object.insert("is_default".to_string(), JsonValue::Bool(false));
    object.insert(
        "name".to_string(),
        JsonValue::String(generated_name.to_string()),
    );
    object.insert(
        "mode".to_string(),
        JsonValue::String(if mode.is_dark() { "dark" } else { "light" }.to_string()),
    );

    let mut colors = match object.remove("colors") {
        Some(JsonValue::Object(colors)) => colors,
        Some(_) => return Err(anyhow!("theme colors is not an object")),
        None => JsonMap::new(),
    };
    let mut highlight = match object.remove("highlight") {
        Some(JsonValue::Object(highlight)) => highlight,
        Some(_) => return Err(anyhow!("theme highlight is not an object")),
        None => JsonMap::new(),
    };

    for (key, raw_value) in &mode_config.overrides {
        let value = raw_value.trim();
        if value.is_empty() {
            continue;
        }
        match find_custom_theme_field(key).map(|field| field.domain) {
            Some(CustomThemeFieldDomain::Brightness) => {}
            Some(CustomThemeFieldDomain::ThemeColor) | None => {
                colors.insert(key.clone(), JsonValue::String(value.to_string()));
            }
            Some(CustomThemeFieldDomain::HighlightColor) => {
                if key.starts_with("syntax.") {
                    set_syntax_override(&mut highlight, key, value)?;
                } else if key.starts_with("editor.") || is_highlight_status_key(key) {
                    highlight.insert(key.clone(), JsonValue::String(value.to_string()));
                }
            }
        }
    }

    object.insert("colors".to_string(), JsonValue::Object(colors));
    object.insert("highlight".to_string(), JsonValue::Object(highlight));

    serde_json::from_value(value).context("deserialize generated custom theme")
}

fn resolve_base_theme(config: &ConfigStore, mode: ThemeMode, cx: &App) -> Rc<ThemeConfig> {
    let base_name = config.custom_theme_base_name(mode);
    if let Some(theme) = ThemeRegistry::global(cx)
        .themes()
        .get(&SharedString::from(base_name.clone()))
        .filter(|theme| theme.mode == mode)
    {
        return theme.clone();
    }

    if mode.is_dark() {
        ThemeRegistry::global(cx).default_dark_theme().clone()
    } else {
        ThemeRegistry::global(cx).default_light_theme().clone()
    }
}

fn build_custom_theme_set(
    config: &ConfigStore,
    cx: &App,
) -> Result<(ThemeSet, ThemeConfig, ThemeConfig)> {
    let draft = config.custom_theme_draft();
    let custom_name = normalized_custom_theme_name(&draft.theme_name);
    let light_name = custom_theme_registry_name(&custom_name, ThemeMode::Light);
    let dark_name = custom_theme_registry_name(&custom_name, ThemeMode::Dark);

    let light = build_custom_theme_config(
        &resolve_base_theme(config, ThemeMode::Light, cx),
        &draft.light,
        &light_name,
        ThemeMode::Light,
    )?;
    let dark = build_custom_theme_config(
        &resolve_base_theme(config, ThemeMode::Dark, cx),
        &draft.dark,
        &dark_name,
        ThemeMode::Dark,
    )?;

    Ok((
        ThemeSet {
            name: custom_name.clone().into(),
            author: Some("ax_shell".into()),
            url: None,
            themes: vec![light.clone(), dark.clone()],
        },
        light,
        dark,
    ))
}

fn write_custom_theme_file(config: &ConfigStore, theme_set: &ThemeSet) -> Result<()> {
    let theme_dir = config
        .theme_dir()
        .or_else(|| ConfigStore::theme_dir_path().ok())
        .ok_or_else(|| anyhow!("could not resolve local theme dir"))?;
    fs::create_dir_all(&theme_dir)
        .with_context(|| format!("failed to create {}", theme_dir.display()))?;
    let path = custom_theme_file_path(&theme_dir, theme_set.name.as_ref());
    let content = serde_json::to_string_pretty(theme_set).context("serialize custom theme file")?;
    fs::write(&path, content).with_context(|| format!("failed to write {}", path.display()))
}

impl AxShell {
    fn current_custom_theme_draft_name(&self) -> String {
        self.config.custom_theme_draft().theme_name
    }

    fn current_custom_theme_registry_name(&self, mode: ThemeMode) -> SharedString {
        custom_theme_registry_name(&self.current_custom_theme_draft_name(), mode).into()
    }

    fn is_current_custom_theme_name(&self, name: &SharedString, mode: ThemeMode) -> bool {
        let generated = self.current_custom_theme_registry_name(mode);
        name == &generated || name.as_ref() == self.config.custom_theme_name()
    }

    pub(crate) fn resolved_custom_theme_base_name(&self, mode: ThemeMode, cx: &App) -> String {
        resolve_base_theme(&self.config, mode, cx).name.to_string()
    }

    pub(crate) fn active_custom_font_brightness(&self, mode: ThemeMode) -> f32 {
        let current_name = if mode.is_dark() {
            &self.appearance.dark_theme_name
        } else {
            &self.appearance.light_theme_name
        };
        if self.is_current_custom_theme_name(current_name, mode) {
            self.config.custom_theme_font_brightness_for_mode(mode)
        } else {
            1.0
        }
    }

    fn resolve_selected_theme(
        &self,
        name: &SharedString,
        mode: ThemeMode,
        cx: &App,
    ) -> Rc<ThemeConfig> {
        if self.is_current_custom_theme_name(name, mode) {
            if let Ok((_, light, dark)) = build_custom_theme_set(&self.config, cx) {
                return Rc::new(if mode.is_dark() { dark } else { light });
            }
        }

        if let Some(theme) = ThemeRegistry::global(cx)
            .themes()
            .get(name)
            .filter(|theme| theme.mode == mode)
        {
            return theme.clone();
        }

        if mode.is_dark() {
            ThemeRegistry::global(cx).default_dark_theme().clone()
        } else {
            ThemeRegistry::global(cx).default_light_theme().clone()
        }
    }

    pub(crate) fn switch_theme_mode(
        &mut self,
        mode: ThemeMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.appearance.follow_system_theme = false;
        self.appearance.theme_mode = mode;
        self.apply_theme_preferences(window, cx);
        self.status = format!("theme mode: {}", cx.theme().mode.name()).into();
        self.persist_theme_preferences();
        cx.notify();
    }

    pub(crate) fn apply_theme(
        &mut self,
        name: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(theme_config) = ThemeRegistry::global(cx).themes().get(&name).cloned() else {
            self.status = format!("theme not found: {name}").into();
            cx.notify();
            return;
        };

        if theme_config.mode.is_dark() {
            self.appearance.dark_theme_name = name.clone();
        } else {
            self.appearance.light_theme_name = name.clone();
        }
        self.apply_theme_preferences(window, cx);
        self.status = format!("theme: {name}").into();
        self.persist_theme_preferences();
        window.refresh();
        cx.notify();
    }

    pub(crate) fn set_custom_theme_base_preset(
        &mut self,
        mode: ThemeMode,
        name: &str,
        cx: &mut Context<Self>,
    ) {
        self.config.set_custom_theme_base_name(mode, name);
        if let Err(err) = self.config.save() {
            self.status = format!("failed to save custom theme base: {err:#}").into();
        } else {
            self.status = format!("custom {} base: {name}", mode.name()).into();
        }
        cx.notify();
    }

    pub(crate) fn set_follow_system_theme(
        &mut self,
        follow: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.appearance.follow_system_theme = follow;
        if follow {
            self.status = "theme mode: system".into();
        } else {
            self.status = format!("theme mode: {}", cx.theme().mode.name()).into();
        }
        self.apply_theme_preferences(window, cx);
        self.persist_theme_preferences();
        cx.notify();
    }

    pub(crate) fn set_display_language(
        &mut self,
        locale: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.config.set_locale(locale);
        let mut active_locale = locale.to_string();
        if active_locale == "system" {
            active_locale = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
            if active_locale.starts_with("zh") {
                active_locale = "zh-CN".to_string();
            } else {
                active_locale = "en".to_string();
            }
        }
        rust_i18n::set_locale(&active_locale);
        gpui_component::set_locale(&active_locale);
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save language preferences: {err:#}");
        }
        window.refresh();
        cx.notify();
    }

    pub(crate) fn apply_theme_preferences(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let light_theme =
            self.resolve_selected_theme(&self.appearance.light_theme_name, ThemeMode::Light, cx);
        let dark_theme =
            self.resolve_selected_theme(&self.appearance.dark_theme_name, ThemeMode::Dark, cx);
        let theme = Theme::global_mut(cx);
        theme.light_theme = light_theme;
        theme.dark_theme = dark_theme;
        theme.font_size = px(self.appearance.ui_font_size);
        set_theme_font_names(theme, &self.appearance.ui_font_family);

        if self.appearance.follow_system_theme {
            Theme::sync_system_appearance(Some(window), cx);
        } else {
            Theme::change(self.appearance.theme_mode, Some(window), cx);
        }
    }

    pub(crate) fn save_custom_appearance(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let previous_draft = self.config.custom_theme_draft();
        let theme_name = self
            .custom_theme_inputs
            .get(CUSTOM_THEME_NAME_INPUT_KEY)
            .expect("custom theme name input missing")
            .read(cx)
            .value()
            .trim()
            .to_string();
        self.config.set_custom_theme_draft_name(&theme_name);

        for mode in custom_theme_modes() {
            for field in custom_theme_field_specs() {
                let input_key = custom_theme_input_key(mode, field.key);
                let value = self
                    .custom_theme_inputs
                    .get(&input_key)
                    .expect("custom theme input missing")
                    .read(cx)
                    .value()
                    .trim()
                    .to_string();

                match field.domain {
                    CustomThemeFieldDomain::Brightness => {
                        let brightness = match value.parse::<f32>() {
                            Ok(value) if (0.6..=1.2).contains(&value) => value,
                            _ => {
                                self.status = format!(
                                    "invalid {} value, use 0.60-1.20",
                                    field.label.to_lowercase()
                                )
                                .into();
                                cx.notify();
                                return;
                            }
                        };
                        self.config
                            .set_custom_theme_font_brightness_for_mode(mode, brightness);
                    }
                    CustomThemeFieldDomain::ThemeColor | CustomThemeFieldDomain::HighlightColor => {
                        if !value.is_empty() && try_parse_color(&value).is_err() {
                            self.status = format!(
                                "invalid color for {}: use hex like #RRGGBB or #RRGGBBAA",
                                field.label
                            )
                            .into();
                            cx.notify();
                            return;
                        }
                        self.config
                            .set_custom_theme_override(mode, field.key, &value);
                    }
                }
            }
        }

        let (theme_set, _, _) = match build_custom_theme_set(&self.config, cx) {
            Ok(themes) => themes,
            Err(err) => {
                self.status = format!("failed to build custom theme: {err:#}").into();
                cx.notify();
                return;
            }
        };

        if let Err(err) = write_custom_theme_file(&self.config, &theme_set) {
            self.status = format!("failed to save custom theme file: {err:#}").into();
            cx.notify();
            return;
        }

        match serde_json::to_string_pretty(&theme_set) {
            Ok(theme_json) => {
                if let Err(err) = ThemeRegistry::global_mut(cx).load_themes_from_str(&theme_json) {
                    tracing::warn!("failed to seed custom theme into registry: {err:#}");
                }
            }
            Err(err) => {
                tracing::warn!("failed to serialize custom theme for registry seed: {err:#}");
            }
        }

        let previous_light =
            custom_theme_registry_name(&previous_draft.theme_name, ThemeMode::Light);
        let previous_dark = custom_theme_registry_name(&previous_draft.theme_name, ThemeMode::Dark);
        let current_light =
            custom_theme_registry_name(&self.current_custom_theme_draft_name(), ThemeMode::Light);
        let current_dark =
            custom_theme_registry_name(&self.current_custom_theme_draft_name(), ThemeMode::Dark);

        if self.appearance.light_theme_name.as_ref() == previous_draft.theme_name
            || self.appearance.light_theme_name.as_ref() == previous_light
        {
            self.appearance.light_theme_name = current_light.clone().into();
        }
        if self.appearance.dark_theme_name.as_ref() == previous_draft.theme_name
            || self.appearance.dark_theme_name.as_ref() == previous_dark
        {
            self.appearance.dark_theme_name = current_dark.clone().into();
        }

        if Theme::global(cx).mode.is_dark() {
            self.appearance.dark_theme_name = current_dark.into();
        } else {
            self.appearance.light_theme_name = current_light.into();
        }

        self.apply_theme_preferences(window, cx);
        self.persist_theme_preferences();
        self.status = "custom theme saved to theme list".into();
        window.refresh();
        cx.notify();
    }

    pub(crate) fn reset_custom_appearance(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.config.reset_custom_theme_draft();
        if let Err(err) = self.config.save() {
            self.status = format!("failed to reset custom theme draft: {err:#}").into();
            cx.notify();
            return;
        }

        if let Some(input) = self.custom_theme_inputs.get(CUSTOM_THEME_NAME_INPUT_KEY) {
            input.update(cx, |input, cx| {
                input.set_value("Custom Theme", window, cx);
            });
        }
        for mode in custom_theme_modes() {
            for field in custom_theme_field_specs() {
                let input_key = custom_theme_input_key(mode, field.key);
                if let Some(input) = self.custom_theme_inputs.get(&input_key) {
                    let reset_value = if field.domain == CustomThemeFieldDomain::Brightness {
                        "1.00"
                    } else {
                        ""
                    };
                    input.update(cx, |input, cx| {
                        input.set_value(reset_value, window, cx);
                    });
                }
            }
        }

        self.status = "custom theme editor reset".into();
        cx.notify();
    }

    pub(crate) fn persist_theme_preferences(&mut self) {
        let theme_mode_str = match self.appearance.theme_mode {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        };
        self.config.set_theme_preferences(
            self.appearance.follow_system_theme,
            theme_mode_str,
            self.appearance.light_theme_name.to_string(),
            self.appearance.dark_theme_name.to_string(),
        );
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save theme preferences: {err:#}");
        }
    }
}
