use std::{collections::HashSet, time::Instant};

use crate::{
    AxShell, PaneLayout,
    app::{
        AxShellWindowKind, LocalFileBrowserState, constants,
        search::SearchState,
        state::{
            appearance::AppearanceState, lifecycle::LifecycleState, monitoring::MonitoringState,
            runtime::RuntimeState,
        },
    },
    config::ConfigStore,
    events::BackendEventSender,
    session::AuthMethod,
};
use gpui::{AppContext as _, Context, SharedString, Window, px};
use gpui_component::{ThemeMode, ThemeRegistry, input::InputState, menu::AppMenuBar};
use rust_i18n::t;

impl AxShell {
    pub(crate) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let (events_tx, events_rx) = crate::events::backend_event_channel();
        Self::new_with_events(
            window,
            cx,
            events_tx,
            events_rx,
            true,
            AxShellWindowKind::Main,
        )
    }

    pub(crate) fn new_with_events(
        window: &mut Window,
        cx: &mut Context<Self>,
        events_tx: BackendEventSender,
        events_rx: tokio::sync::mpsc::Receiver<crate::events::BackendEvent>,
        recover_interrupted_transfers: bool,
        window_kind: AxShellWindowKind,
    ) -> Self {
        let is_detached_workspace = window_kind.is_detached();
        let host_input = cx.new(|cx| InputState::new(window, cx).placeholder(t!("host")));
        let session_name_input =
            cx.new(|cx| InputState::new(window, cx).placeholder(t!("connection_name_optional")));
        let session_group_input =
            cx.new(|cx| InputState::new(window, cx).placeholder(t!("session_group_optional")));
        let port_input = cx.new(|cx| InputState::new(window, cx).default_value("22"));
        let user_input = cx.new(|cx| InputState::new(window, cx).default_value("root"));
        let password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("password"))
                .masked(true)
        });
        let key_path_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("~/.ssh/id_ed25519"));
        let key_inline_input = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .rows(5)
                .placeholder("-----BEGIN OPENSSH PRIVATE KEY-----")
        });
        let passphrase_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("private_key_passphrase_optional"))
                .masked(true)
        });
        let proxy_host_input =
            cx.new(|cx| InputState::new(window, cx).placeholder(t!("proxy_host").to_string()));
        let proxy_port_input =
            cx.new(|cx| InputState::new(window, cx).placeholder(t!("proxy_port").to_string()));
        let proxy_user_input =
            cx.new(|cx| InputState::new(window, cx).placeholder(t!("proxy_user").to_string()));
        let proxy_password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("proxy_password").to_string())
                .masked(true)
        });
        let session_sftp_path_input =
            cx.new(|cx| InputState::new(window, cx).placeholder(t!("sftp_path").to_string()));
        let serial_port_input =
            cx.new(|cx| InputState::new(window, cx).placeholder(t!("serial_port").to_string()));
        let serial_baud_rate_input =
            cx.new(|cx| InputState::new(window, cx).default_value("115200"));
        let serial_data_bits_input = cx.new(|cx| InputState::new(window, cx).default_value("8"));
        let serial_parity_input = cx.new(|cx| InputState::new(window, cx).default_value("none"));
        let serial_stop_bits_input = cx.new(|cx| InputState::new(window, cx).default_value("1"));
        let serial_flow_control_input =
            cx.new(|cx| InputState::new(window, cx).default_value("none"));
        let sftp_path_input = cx.new(|cx| InputState::new(window, cx).default_value("/"));
        let sftp_new_folder_input =
            cx.new(|cx| InputState::new(window, cx).placeholder(t!("new_folder").to_string()));
        let search_input =
            cx.new(|cx| InputState::new(window, cx).placeholder(t!("search").to_string()));
        let config = ConfigStore::load().unwrap_or_else(|err| {
            tracing::warn!(
                component = "config",
                operation = "load",
                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                "Failed to load configuration; using in-memory defaults"
            );
            ConfigStore::in_memory()
        });
        if let Err(err) =
            crate::app::theme::ensure_embedded_font_family_loaded(config.ui_font_family(), cx)
        {
            tracing::warn!(
                component = "theme",
                operation = "load_configured_ui_font",
                font_family = config.ui_font_family(),
                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                "Failed to load configured embedded UI font"
            );
        }
        // SFTP is not part of the initial terminal workspace. Keep its icon cache
        // and local browser inert until a valid SFTP page is actually opened.
        let file_icons = crate::platform::file_icons::FileIconCache::default();
        let default_local_sftp_path_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(Self::home_local_browser_dir())
                .default_value(config.default_local_sftp_path())
        });
        // Do not validate or enumerate the configured local SFTP directory during
        // window construction. The first SFTP route restores the real location.
        let default_local_dir = Self::home_local_browser_dir();
        let local_sftp_path_input =
            cx.new(|cx| InputState::new(window, cx).default_value(default_local_dir.clone()));
        let rayon_threads_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("rayon_threads_input_placeholder").to_string())
                .default_value(config.rayon_threads().to_string())
        });
        let local_shell_profile = config.default_local_shell_profile();
        let local_shell_profile_name_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("local_shell_profile_name_placeholder").to_string())
                .default_value(local_shell_profile.name.clone())
        });
        let local_shell_profile_program_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("local_shell_program_placeholder").to_string())
                .default_value(local_shell_profile.program.clone())
        });
        let local_shell_profile_args_input = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .rows(3)
                .placeholder(t!("local_shell_args_placeholder").to_string())
                .default_value(local_shell_profile.args.join("\n"))
        });
        let app_menu_bar = if cfg!(any(target_os = "windows", target_os = "linux")) {
            Some(AppMenuBar::new(cx))
        } else {
            None
        };
        let global_proxy_host_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("proxy_host").to_string())
                .default_value(config.global_proxy_host())
        });
        let global_proxy_port_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("proxy_port").to_string())
                .default_value(
                    config
                        .global_proxy_port()
                        .map(|p| p.to_string())
                        .unwrap_or_default(),
                )
        });
        let global_proxy_user_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("proxy_user").to_string())
                .default_value(config.global_proxy_user())
        });
        let global_proxy_password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("proxy_password").to_string())
                .masked(true)
                .default_value(config.global_proxy_password())
        });
        let ssh_retry_count_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("2")
                .default_value(config.ssh_connect_retry_count().to_string())
        });
        let ssh_retry_delays_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("500, 1500")
                .default_value(
                    config
                        .ssh_connect_retry_delays_ms()
                        .into_iter()
                        .map(|delay| delay.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                )
        });
        let xquartz_app_path_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(crate::platform::x_server::default_app_path())
                .default_value(config.local_x_server_app_path())
        });
        let sync_endpoint_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("https://dav.example.com/ax_shell/")
                .default_value(config.sync_endpoint())
        });
        let sync_username_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("sync_username").to_string())
                .default_value(config.sync_username())
        });
        let sync_webdav_password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("sync_webdav_password").to_string())
                .masked(true)
        });
        let sync_s3_endpoint_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("https://s3.example.com")
                .default_value(config.sync_s3_endpoint())
        });
        let sync_s3_region_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("us-east-1")
                .default_value(config.sync_s3_region())
        });
        let sync_s3_bucket_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("sync_s3_bucket").to_string())
                .default_value(config.sync_s3_bucket())
        });
        let sync_s3_object_key_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("ax_shell-sync.json")
                .default_value(config.sync_s3_object_key())
        });
        let sync_s3_access_key_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder(t!("sync_s3_access_key").to_string())
        });
        let sync_s3_secret_key_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("sync_s3_secret_key").to_string())
                .masked(true)
        });
        let sync_s3_session_token_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("sync_s3_session_token").to_string())
                .masked(true)
        });
        let sync_encryption_password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(t!("sync_encryption_password").to_string())
                .masked(true)
        });
        let custom_theme_save_path_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("themes/")
                .default_value(config.custom_theme_save_path())
        });
        let custom_theme_draft = config.custom_theme_draft();
        let custom_theme_draft_name = custom_theme_draft.theme_name.clone();
        let mut custom_theme_inputs = std::collections::HashMap::new();
        custom_theme_inputs.insert(
            crate::app::theme::custom_theme_name_input_key().to_string(),
            cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Custom Theme")
                    .default_value(custom_theme_draft_name.clone())
            }),
        );
        for mode in crate::app::theme::custom_theme_modes() {
            let mode_config = if mode.is_dark() {
                &custom_theme_draft.dark
            } else {
                &custom_theme_draft.light
            };
            for section in crate::app::theme::CUSTOM_THEME_SECTION_SPECS {
                for field in section.fields {
                    let default_value = mode_config
                        .overrides
                        .get(field.key)
                        .cloned()
                        .unwrap_or_default();
                    let input_key = crate::app::theme::custom_theme_input_key(mode, field.key);
                    let placeholder = field.placeholder.to_string();
                    custom_theme_inputs.insert(
                        input_key,
                        cx.new(|cx| {
                            InputState::new(window, cx)
                                .placeholder(placeholder.clone())
                                .default_value(default_value.clone())
                        }),
                    );
                }
            }
        }
        let saved_group_name_input =
            cx.new(|cx| InputState::new(window, cx).placeholder(t!("group_name")));
        let local_entries = Vec::new();
        let local_status = default_local_dir.clone();

        let mut _subscriptions = vec![
            cx.subscribe_in(&host_input, window, Self::on_input_event),
            cx.subscribe_in(&session_name_input, window, Self::on_input_event),
            cx.subscribe_in(&session_group_input, window, Self::on_input_event),
            cx.subscribe_in(&port_input, window, Self::on_input_event),
            cx.subscribe_in(&user_input, window, Self::on_input_event),
            cx.subscribe_in(&password_input, window, Self::on_input_event),
            cx.subscribe_in(&key_path_input, window, Self::on_input_event),
            cx.subscribe_in(&key_inline_input, window, Self::on_input_event),
            cx.subscribe_in(&passphrase_input, window, Self::on_input_event),
            cx.subscribe_in(&proxy_host_input, window, Self::on_input_event),
            cx.subscribe_in(&proxy_port_input, window, Self::on_input_event),
            cx.subscribe_in(&proxy_user_input, window, Self::on_input_event),
            cx.subscribe_in(&proxy_password_input, window, Self::on_input_event),
            cx.subscribe_in(&session_sftp_path_input, window, Self::on_input_event),
            cx.subscribe_in(&serial_port_input, window, Self::on_input_event),
            cx.subscribe_in(&serial_baud_rate_input, window, Self::on_input_event),
            cx.subscribe_in(&serial_data_bits_input, window, Self::on_input_event),
            cx.subscribe_in(&serial_parity_input, window, Self::on_input_event),
            cx.subscribe_in(&serial_stop_bits_input, window, Self::on_input_event),
            cx.subscribe_in(&serial_flow_control_input, window, Self::on_input_event),
            cx.subscribe_in(&ssh_retry_count_input, window, Self::on_input_event),
            cx.subscribe_in(&ssh_retry_delays_input, window, Self::on_input_event),
            cx.subscribe_in(&rayon_threads_input, window, Self::on_input_event),
            cx.subscribe_in(&default_local_sftp_path_input, window, Self::on_input_event),
            cx.subscribe_in(&xquartz_app_path_input, window, Self::on_input_event),
            cx.subscribe_in(&sftp_path_input, window, Self::on_input_event),
            cx.subscribe_in(&local_sftp_path_input, window, Self::on_input_event),
            cx.subscribe_in(&sftp_new_folder_input, window, Self::on_input_event),
            cx.subscribe_in(&search_input, window, Self::on_input_event),
            cx.subscribe_in(&sync_endpoint_input, window, Self::on_input_event),
            cx.subscribe_in(&sync_username_input, window, Self::on_input_event),
            cx.subscribe_in(&sync_webdav_password_input, window, Self::on_input_event),
            cx.subscribe_in(&sync_s3_endpoint_input, window, Self::on_input_event),
            cx.subscribe_in(&sync_s3_region_input, window, Self::on_input_event),
            cx.subscribe_in(&sync_s3_bucket_input, window, Self::on_input_event),
            cx.subscribe_in(&sync_s3_object_key_input, window, Self::on_input_event),
            cx.subscribe_in(&sync_s3_access_key_input, window, Self::on_input_event),
            cx.subscribe_in(&sync_s3_secret_key_input, window, Self::on_input_event),
            cx.subscribe_in(&sync_s3_session_token_input, window, Self::on_input_event),
            cx.subscribe_in(
                &sync_encryption_password_input,
                window,
                Self::on_input_event,
            ),
            cx.subscribe_in(&custom_theme_save_path_input, window, Self::on_input_event),
            cx.subscribe_in(&saved_group_name_input, window, Self::on_input_event),
        ];
        _subscriptions.extend(
            custom_theme_inputs
                .values()
                .map(|input| cx.subscribe_in(input, window, Self::on_input_event)),
        );
        _subscriptions
            .push(cx.observe_window_activation(window, Self::on_window_activation_changed));
        _subscriptions.push(cx.observe_window_bounds(window, Self::on_window_bounds_changed));

        let workspace_panels = cx.new(|_| crate::app::resizable::ResizableState::default());
        let body_panels = cx.new(|_| crate::app::resizable::ResizableState::default());
        let sftp_transfer_panels = cx.new(|_| crate::app::resizable::ResizableState::default());
        let system = Default::default();
        let default_light_theme_name = ThemeRegistry::global(cx).default_light_theme().name.clone();
        let default_dark_theme_name = ThemeRegistry::global(cx).default_dark_theme().name.clone();
        let follow_system_theme = config.follow_system_theme();

        let theme_mode = match config.theme_mode() {
            "light" => ThemeMode::Light,
            "dark" => ThemeMode::Dark,
            _ => ThemeMode::Light,
        };
        let migrated_light_custom_name = crate::app::theme::custom_theme_registry_name(
            &custom_theme_draft.theme_name,
            ThemeMode::Light,
        );
        let migrated_dark_custom_name = crate::app::theme::custom_theme_registry_name(
            &custom_theme_draft.theme_name,
            ThemeMode::Dark,
        );
        let active_theme_profile = config.active_theme_profile().cloned();
        let light_theme_name = if config.light_theme_name().is_empty() {
            active_theme_profile
                .as_ref()
                .filter(|profile| !profile.light_theme_name.trim().is_empty())
                .map(|profile| profile.light_theme_name.clone().into())
                .unwrap_or(default_light_theme_name)
        } else if config.light_theme_name() == custom_theme_draft.theme_name
            || config.light_theme_name() == config.custom_theme_name()
        {
            migrated_light_custom_name.into()
        } else {
            config.light_theme_name().into()
        };
        let dark_theme_name = if config.dark_theme_name().is_empty() {
            active_theme_profile
                .as_ref()
                .filter(|profile| !profile.dark_theme_name.trim().is_empty())
                .map(|profile| profile.dark_theme_name.clone().into())
                .unwrap_or(default_dark_theme_name)
        } else if config.dark_theme_name() == custom_theme_draft.theme_name
            || config.dark_theme_name() == config.custom_theme_name()
        {
            migrated_dark_custom_name.into()
        } else {
            config.dark_theme_name().into()
        };

        let configured_locale = config.locale();
        let mut active_locale = configured_locale.to_string();
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
        let ui_font_family: SharedString = config.ui_font_family().into();
        let terminal_font_family: SharedString = config.terminal_font_family().into();
        let last_sidebar_width = Some(px(config
            .workspace_panels()
            .and_then(|s| s.first().copied())
            .unwrap_or(constants::SIDEBAR_WIDTH)));
        let mut this = Self {
            focus_handle: cx.focus_handle(),
            selector_focus_handle: cx.focus_handle(),
            host_input,
            session_name_input,
            session_group_input,
            port_input,
            user_input,
            password_input,
            key_path_input,
            key_inline_input,
            passphrase_input,
            ssh_proxy_type: "none".to_string(),
            proxy_host_input,
            proxy_port_input,
            proxy_user_input,
            proxy_password_input,
            session_sftp_path_input,
            session_x11_forwarding: true,
            session_legacy_ssh_compatibility: false,
            ssh_advanced_options_visible: false,
            global_proxy_type: config.global_proxy_type().to_string(),
            global_proxy_host_input,
            global_proxy_port_input,
            global_proxy_user_input,
            global_proxy_password_input,
            ssh_retry_count_input,
            ssh_retry_delays_input,
            rayon_threads_input,
            local_shell_profile_name_input,
            local_shell_profile_program_input,
            local_shell_profile_args_input,
            default_local_sftp_path_input,
            xquartz_app_path_input,
            sync_endpoint_input,
            sync_username_input,
            sync_webdav_password_input,
            sync_s3_endpoint_input,
            sync_s3_region_input,
            sync_s3_bucket_input,
            sync_s3_object_key_input,
            sync_s3_access_key_input,
            sync_s3_secret_key_input,
            sync_s3_session_token_input,
            sync_encryption_password_input,
            custom_theme_save_path_input,
            custom_theme_inputs,
            sync_in_progress: false,
            sync_status: t!("sync_not_run").into(),
            sftp_path_input,
            local_sftp_path_input,
            serial_port_input,
            serial_baud_rate_input,
            serial_data_bits_input,
            serial_parity_input,
            serial_stop_bits_input,
            serial_flow_control_input,
            session_kind: crate::session::SessionKind::Ssh,
            available_serial_ports: Vec::new(),
            ssh_auth_method: AuthMethod::Password,
            editing_session_id: None,
            session_shortcut: String::new(),
            recording_session_shortcut: false,
            session_shortcut_error: None,
            session_import_error: None,
            appearance: AppearanceState {
                follow_system_theme,
                theme_mode,
                light_theme_name,
                dark_theme_name,
                ui_font_size: config.ui_font_size(),
                terminal_font_size: config.terminal_font_size(),
                ui_font_brightness: config.ui_font_brightness(),
                terminal_font_brightness: config.terminal_font_brightness(),
                terminal_font_metrics: crate::app::TerminalFontMetrics::fallback(
                    config.terminal_font_size(),
                ),
                terminal_zoom_accumulator: 0.0,
                ui_font_family,
                terminal_font_family,
                cursor_style: config.cursor_style(),
                last_theme_sync: Instant::now(),
            },
            lifecycle: LifecycleState::new(window.is_window_active()),
            tabs: Vec::new(),
            active_tab: None,
            tab_groups: Vec::new(),
            workspace_group_instance_counts: std::collections::HashMap::new(),
            active_group: None,
            pane_root: PaneLayout::Single(String::new()),
            focused_pane_path: Vec::new(),
            terminal_panel_bounds: None,
            selector_selection: 0,
            workspace_panels,
            body_panels,
            sftp_transfer_panels,
            is_layout_reset: false,
            terminal_scrollbars: std::collections::HashMap::new(),
            remote_files_scroll_handle: gpui::UniformListScrollHandle::new(),
            local_files_scroll_handle: gpui::UniformListScrollHandle::new(),
            disk_scroll_handle: gpui::ScrollHandle::new(),
            tabs_scroll_handle: gpui::ScrollHandle::new(),
            selector_scroll_handle: gpui::UniformListScrollHandle::new(),
            saved_scroll_handle: gpui::UniformListScrollHandle::new(),
            saved_group_name_input,
            connection_scroll_handle: gpui::ScrollHandle::new(),
            connection_progress: None,
            terminal_password_prompt: None,
            terminal_password_retry_tabs: HashSet::new(),
            pending_sftp_path_sync: Some("/".into()),
            pending_sftp_selection_path: None,
            pending_sftp_terminal_cwd_tab: None,
            pending_local_sftp_path_sync: None,
            local_file_browser: LocalFileBrowserState {
                current_path: default_local_dir.clone(),
                status: local_status,
                entries: local_entries,
                selected_path: None,
                selected_entries: HashSet::new(),
            },
            file_icons,
            sftp_context_menu: None,
            sftp_transfer_context_menu: None,
            saved_group_context_menu: None,
            saved_session_context_menu: None,
            sftp_creating_folder: false,
            sftp_close_remember_choice: false,
            sftp_close_confirm_group_id: None,
            sftp_edit_close_group_id: None,
            sftp_edit_upload_requests: std::collections::VecDeque::new(),
            sftp_edit_upload_request: None,
            host_key_verification_requests: std::collections::VecDeque::new(),
            sftp_overwrite_requests: std::collections::VecDeque::new(),
            sftp_replace_all_for_run: false,
            sftp_new_folder_input,
            sftp_delete_scroll_handle: gpui::ScrollHandle::new(),
            show_hidden_files: config.show_hidden_files(),
            remote_sftp_sort_column: crate::app::SftpSortColumn::Name,
            remote_sftp_sort_direction: crate::app::SortDirection::Asc,
            local_sftp_sort_column: crate::app::SftpSortColumn::Name,
            local_sftp_sort_direction: crate::app::SortDirection::Asc,
            sftp_transfer_tab: crate::app::SftpTransferTab::Active,
            sftp_transfer_scroll_handle: gpui::UniformListScrollHandle::new(),
            sftp_transfer_files_scroll_handle: gpui::UniformListScrollHandle::new(),
            transfers: if is_detached_workspace {
                // The transferred workspace replaces this collection immediately.
                // Avoid cloning persisted transfer history into detached windows.
                Vec::new()
            } else {
                let mut transfers = config.transfers();
                let now = crate::sftp::unix_timestamp_secs();
                if recover_interrupted_transfers {
                    for t in &mut transfers {
                        if matches!(
                            t.state,
                            crate::sftp::TransferState::Running
                                | crate::sftp::TransferState::Paused
                        ) {
                            let reason = t!("zombie_reason").to_string();
                            t.state = crate::sftp::TransferState::Zombie(reason.clone());
                            for file in &mut t.files {
                                if !file.state.is_terminal() {
                                    file.state =
                                        crate::sftp::TransferFileState::Interrupted(reason.clone());
                                }
                            }
                            if t.finished_at.is_none() {
                                t.finished_at = Some(now);
                            }
                        }
                    }
                }
                transfers
            },
            show_transfers_dialog: false,
            terminal_bounds: std::collections::HashMap::new(),
            terminal_selecting: false,
            terminal_composition: None,
            terminal_frozen_selection: None,
            dragging_splitter: None,
            drag_split_origin: None,
            sidebar_collapsed: config.sidebar_collapsed(),
            collapsed_saved_scroll_handle: gpui::UniformListScrollHandle::new(),
            status: "ready".into(),
            active_title_bar_style: config.effective_title_bar_style(),
            config,
            app_menu_bar,
            recording_action: None,
            active_dialog: None,
            renaming_saved_group: None,
            expanded_saved_groups: HashSet::new(),
            workspace_page: crate::app::WorkspacePage::Terminal,
            settings_page_open: false,
            settings_page_generation: 0,
            settings_initial_page: 0,
            settings_close_remember_choice: false,
            keybind_error: None,
            keybinds_suspended: false,
            monitoring: MonitoringState {
                status: None,
                sampler: None,
                system,
                cpu_history: Vec::with_capacity(20),
                net_rx_history: Vec::with_capacity(20),
                net_tx_history: Vec::with_capacity(20),
                // Preserve an immediate first update when the main-window
                // dashboard is visible, while keeping the sampler lazy.
                last_sample: Instant::now() - crate::monitoring::SystemSampler::interval(),
                system_tab_id: None,
                remote_sample_generation: 0,
                remote_sample_in_flight: None,
            },
            search: SearchState {
                input: search_input,
                active: false,
                query: String::new(),
                matches: Vec::new(),
                current: 0,
                target_tab: None,
                bar_bounds: None,
            },
            sftp_handles: std::collections::HashMap::new(),
            sftp_last_activity: std::collections::HashMap::new(),
            runtime_state: RuntimeState::new(events_rx, events_tx),
            last_window_size: None,
            last_sidebar_width,
            should_move_window: false,
            persist_window_layout: !is_detached_workspace,
            is_detached_workspace,
            detached_window_title: None,
            hovered_url: None,
            cmd_ctrl_pressed: false,
            _subscriptions,
        };

        this.sync_custom_theme_inputs_from_draft(window, cx);
        this.apply_theme_preferences(window, cx);
        this.start_event_pump(cx);
        this
    }

    pub(crate) fn install_workspace_transfer(&mut self, transfer: crate::app::WorkspaceTransfer) {
        let crate::app::WorkspaceTransfer {
            group,
            tabs,
            sftp_handle,
            sftp_last_activity,
            connection_progress,
            terminal_password_prompt,
            terminal_password_retry_tabs,
            transfers,
            active_tab,
            focused_pane_path,
            workspace_page,
            runtime,
        } = transfer;
        let tab_ids = group
            .pane_root
            .tab_ids()
            .into_iter()
            .filter(|id| !id.is_empty())
            .map(str::to_string)
            .collect::<Vec<_>>();
        self.runtime_state
            .events_tx
            .register_route(group.id.clone());
        self.runtime_state
            .events_tx
            .register_routes(tab_ids.iter().map(String::as_str));
        if let Some(runtime) = runtime {
            self.runtime_state.adopt_shared_runtime(runtime);
        }
        self.workspace_group_instance_counts
            .entry(group.title.clone())
            .and_modify(|next| *next = (*next).max(group.instance_number))
            .or_insert(group.instance_number);
        self.detached_window_title = Some(format!("{} #{}", group.title, group.instance_number));
        self.active_tab = active_tab.or_else(|| tab_ids.first().cloned());
        self.focused_pane_path = focused_pane_path;
        self.pane_root = group.pane_root.clone();
        self.active_group = Some(group.id.clone());
        let _ = workspace_page;
        self.workspace_page = crate::app::WorkspacePage::Terminal;
        self.tab_groups.push(group);
        self.tabs = tabs;
        if let Some(handle) = sftp_handle {
            let group_id = self.active_group.clone().expect("transferred group exists");
            self.sftp_handles.insert(group_id, handle);
        }
        if let Some(last_activity) = sftp_last_activity {
            let group_id = self.active_group.clone().expect("transferred group exists");
            self.sftp_last_activity.insert(group_id, last_activity);
        }
        self.connection_progress = connection_progress;
        self.terminal_password_prompt = terminal_password_prompt;
        self.terminal_password_retry_tabs = terminal_password_retry_tabs;
        self.transfers = transfers;
        self.persist_window_layout = false;
        self.is_detached_workspace = true;
    }
}
