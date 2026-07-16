pub(crate) mod actions;
mod config_sync;
pub(crate) mod constants;
pub mod dialogs;
mod hover;
mod input;
mod lifecycle;
mod pane;
pub mod resizable;
mod search;
mod session_ui;
mod sftp;
mod state;
mod terminal;
pub mod theme;
pub mod views;
mod workspace;

pub(crate) use input::{app_menu, keybinding_recorder};
pub(crate) use lifecycle::startup;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use crate::app::resizable::ResizableState;
use gpui::{Bounds, Entity, FocusHandle, Pixels, Point, SharedString, UniformListScrollHandle};
use gpui_component::{input::InputState, menu::AppMenuBar};

use crate::{
    config::ConfigStore,
    session::{AuthMethod, SessionKind},
    terminal::{TerminalComposition, TerminalFrozenSelection, TerminalTab},
};
use state::{
    appearance::AppearanceState, lifecycle::LifecycleState, monitoring::MonitoringState,
    runtime::RuntimeState,
};

pub(crate) use state::runtime::RuntimeTaskTracker;
pub(crate) use state::runtime::SharedRuntime;

use search::SearchState;

pub(crate) use dialogs::DialogKind;
pub(crate) use pane::PaneLayout;
pub(crate) use session_ui::{ConnectionProgress, SelectorEntry, TerminalPasswordPrompt};
pub(crate) use sftp::{
    LocalFileBrowserState, LocalFileEntry, SftpContextMenuState, SftpContextMenuTarget,
    SftpEditSession, SftpEditUploadRequest, SftpSortColumn, SftpTransferContextMenuState,
    SftpTransferTab, SftpUiState, SortDirection,
};
pub(crate) use terminal::{
    HoverTargetKind, HoveredUrl, TerminalFontMetrics, TerminalScrollbarHandle,
    terminal_link_activation_modifier_pressed, terminal_link_visual_active,
};
pub(crate) use workspace::{TabGroup, WorkspacePage, workspace_group_tab_label};

pub(crate) struct WorkspaceTransfer {
    pub(crate) group: TabGroup,
    pub(crate) tabs: Vec<TerminalTab>,
    pub(crate) sftp_handle: Option<crate::sftp::SftpHandle>,
    pub(crate) sftp_last_activity: Option<Instant>,
    pub(crate) connection_progress: Option<ConnectionProgress>,
    pub(crate) terminal_password_prompt: Option<TerminalPasswordPrompt>,
    pub(crate) terminal_password_retry_tabs: HashSet<String>,
    pub(crate) transfers: Vec<crate::sftp::Transfer>,
    pub(crate) active_tab: Option<String>,
    pub(crate) focused_pane_path: Vec<usize>,
    pub(crate) workspace_page: WorkspacePage,
    pub(crate) runtime: Option<SharedRuntime>,
}

pub(crate) struct MainWorkspace {
    pub(crate) view: Entity<AxShell>,
}

impl gpui::Global for MainWorkspace {}

/// Selects initialization work appropriate for the native window being opened.
///
/// A detached workspace only renders its transferred terminal workspace, so it
/// can skip main-window-only SFTP and file-icon prewarming from the outset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AxShellWindowKind {
    Main,
    Detached,
}

impl AxShellWindowKind {
    pub(crate) const fn is_detached(self) -> bool {
        matches!(self, Self::Detached)
    }
}

#[derive(Clone)]
pub(crate) struct SavedSessionContextMenuState {
    pub(crate) session_id: String,
    pub(crate) position: Point<Pixels>,
}

#[derive(Clone)]
pub(crate) struct SavedGroupContextMenuState {
    pub(crate) group_name: String,
    pub(crate) position: Point<Pixels>,
}

pub(crate) struct AxShell {
    pub(crate) focus_handle: FocusHandle,
    pub(crate) selector_focus_handle: FocusHandle,
    pub(crate) host_input: Entity<InputState>,
    pub(crate) session_name_input: Entity<InputState>,
    pub(crate) session_group_input: Entity<InputState>,
    pub(crate) port_input: Entity<InputState>,
    pub(crate) user_input: Entity<InputState>,
    pub(crate) password_input: Entity<InputState>,
    pub(crate) key_path_input: Entity<InputState>,
    pub(crate) key_inline_input: Entity<InputState>,
    pub(crate) passphrase_input: Entity<InputState>,
    pub(crate) ssh_proxy_type: String,
    pub(crate) proxy_host_input: Entity<InputState>,
    pub(crate) proxy_port_input: Entity<InputState>,
    pub(crate) proxy_user_input: Entity<InputState>,
    pub(crate) proxy_password_input: Entity<InputState>,
    pub(crate) session_sftp_path_input: Entity<InputState>,
    pub(crate) serial_port_input: Entity<InputState>,
    pub(crate) serial_baud_rate_input: Entity<InputState>,
    pub(crate) serial_data_bits_input: Entity<InputState>,
    pub(crate) serial_parity_input: Entity<InputState>,
    pub(crate) serial_stop_bits_input: Entity<InputState>,
    pub(crate) serial_flow_control_input: Entity<InputState>,
    pub(crate) session_kind: SessionKind,
    pub(crate) available_serial_ports: Vec<String>,
    pub(crate) session_x11_forwarding: bool,
    pub(crate) session_legacy_ssh_compatibility: bool,
    pub(crate) ssh_advanced_options_visible: bool,
    pub(crate) session_shortcut: String,
    pub(crate) recording_session_shortcut: bool,
    pub(crate) session_shortcut_error: Option<String>,
    pub(crate) session_import_error: Option<String>,
    pub(crate) global_proxy_type: String,
    pub(crate) global_proxy_host_input: Entity<InputState>,
    pub(crate) global_proxy_port_input: Entity<InputState>,
    pub(crate) global_proxy_user_input: Entity<InputState>,
    pub(crate) global_proxy_password_input: Entity<InputState>,
    pub(crate) ssh_retry_count_input: Entity<InputState>,
    pub(crate) ssh_retry_delays_input: Entity<InputState>,
    pub(crate) rayon_threads_input: Entity<InputState>,
    pub(crate) local_shell_profile_name_input: Entity<InputState>,
    pub(crate) local_shell_profile_program_input: Entity<InputState>,
    pub(crate) local_shell_profile_args_input: Entity<InputState>,
    pub(crate) default_local_sftp_path_input: Entity<InputState>,
    pub(crate) xquartz_app_path_input: Entity<InputState>,
    pub(crate) sync_endpoint_input: Entity<InputState>,
    pub(crate) sync_username_input: Entity<InputState>,
    pub(crate) sync_webdav_password_input: Entity<InputState>,
    pub(crate) sync_s3_endpoint_input: Entity<InputState>,
    pub(crate) sync_s3_region_input: Entity<InputState>,
    pub(crate) sync_s3_bucket_input: Entity<InputState>,
    pub(crate) sync_s3_object_key_input: Entity<InputState>,
    pub(crate) sync_s3_access_key_input: Entity<InputState>,
    pub(crate) sync_s3_secret_key_input: Entity<InputState>,
    pub(crate) sync_s3_session_token_input: Entity<InputState>,
    pub(crate) sync_encryption_password_input: Entity<InputState>,
    pub(crate) custom_theme_save_path_input: Entity<InputState>,
    pub(crate) custom_theme_inputs: HashMap<String, Entity<InputState>>,
    pub(crate) sync_in_progress: bool,
    pub(crate) sync_status: SharedString,
    pub(crate) sftp_path_input: Entity<InputState>,
    pub(crate) local_sftp_path_input: Entity<InputState>,
    pub(crate) ssh_auth_method: AuthMethod,
    pub(crate) editing_session_id: Option<String>,
    pub(crate) appearance: AppearanceState,
    pub(crate) lifecycle: LifecycleState,
    pub(crate) tabs: Vec<TerminalTab>,
    pub(crate) active_tab: Option<String>,
    pub(crate) tab_groups: Vec<TabGroup>,
    pub(crate) workspace_group_instance_counts: HashMap<String, usize>,
    pub(crate) active_group: Option<String>,
    pub(crate) selector_selection: usize,
    pub(crate) workspace_panels: Entity<ResizableState>,
    pub(crate) body_panels: Entity<ResizableState>,
    pub(crate) sftp_transfer_panels: Entity<ResizableState>,
    pub(crate) is_layout_reset: bool,
    pub(crate) terminal_scrollbars: HashMap<String, TerminalScrollbarHandle>,
    pub(crate) remote_files_scroll_handle: UniformListScrollHandle,
    pub(crate) local_files_scroll_handle: UniformListScrollHandle,
    pub(crate) disk_scroll_handle: gpui::ScrollHandle,
    pub(crate) tabs_scroll_handle: gpui::ScrollHandle,
    pub(crate) selector_scroll_handle: UniformListScrollHandle,
    pub(crate) saved_scroll_handle: UniformListScrollHandle,
    pub(crate) saved_group_name_input: Entity<InputState>,
    pub(crate) connection_scroll_handle: gpui::ScrollHandle,
    pub(crate) connection_progress: Option<ConnectionProgress>,
    pub(crate) terminal_password_prompt: Option<TerminalPasswordPrompt>,
    pub(crate) terminal_password_retry_tabs: HashSet<String>,
    pub(crate) pending_sftp_path_sync: Option<String>,
    pub(crate) pending_sftp_selection_path: Option<String>,
    pub(crate) pending_sftp_terminal_cwd_tab: Option<String>,
    pub(crate) pending_local_sftp_path_sync: Option<String>,
    pub(crate) local_file_browser: LocalFileBrowserState,
    pub(crate) file_icons: crate::platform::file_icons::FileIconCache,
    pub(crate) sftp_context_menu: Option<SftpContextMenuState>,
    pub(crate) sftp_transfer_context_menu: Option<SftpTransferContextMenuState>,
    pub(crate) saved_group_context_menu: Option<SavedGroupContextMenuState>,
    pub(crate) saved_session_context_menu: Option<SavedSessionContextMenuState>,
    pub(crate) sftp_creating_folder: bool,
    pub(crate) sftp_close_remember_choice: bool,
    pub(crate) sftp_close_confirm_group_id: Option<String>,
    pub(crate) sftp_edit_close_group_id: Option<String>,
    pub(crate) sftp_edit_upload_requests: VecDeque<SftpEditUploadRequest>,
    pub(crate) sftp_edit_upload_request: Option<SftpEditUploadRequest>,
    pub(crate) host_key_verification_requests:
        VecDeque<crate::backend::host_key::HostKeyVerificationRequest>,
    pub(crate) sftp_overwrite_requests: VecDeque<crate::sftp::SftpOverwriteRequest>,
    pub(crate) sftp_replace_all_for_run: bool,
    pub(crate) sftp_new_folder_input: Entity<InputState>,
    pub(crate) sftp_delete_scroll_handle: gpui::ScrollHandle,
    pub(crate) show_hidden_files: bool,
    pub(crate) remote_sftp_sort_column: SftpSortColumn,
    pub(crate) remote_sftp_sort_direction: SortDirection,
    pub(crate) local_sftp_sort_column: SftpSortColumn,
    pub(crate) local_sftp_sort_direction: SortDirection,
    pub(crate) sftp_transfer_tab: SftpTransferTab,
    pub(crate) sftp_transfer_scroll_handle: UniformListScrollHandle,
    pub(crate) sftp_transfer_files_scroll_handle: UniformListScrollHandle,
    pub(crate) transfers: Vec<crate::sftp::Transfer>,
    pub(crate) show_transfers_dialog: bool,
    pub(crate) pane_root: PaneLayout,
    pub(crate) focused_pane_path: Vec<usize>,
    pub(crate) terminal_panel_bounds: Option<Bounds<Pixels>>,
    pub(crate) terminal_bounds: HashMap<String, Bounds<Pixels>>,
    pub(crate) terminal_selecting: bool,
    pub(crate) dragging_splitter: Option<(Vec<usize>, usize)>, // (parent_path, child_index)
    pub(crate) drag_split_origin: Option<gpui::Point<Pixels>>,
    pub(crate) terminal_composition: Option<TerminalComposition>,
    pub(crate) terminal_frozen_selection: Option<TerminalFrozenSelection>,
    pub(crate) sidebar_collapsed: bool,
    pub(crate) collapsed_saved_scroll_handle: UniformListScrollHandle,
    pub(crate) status: SharedString,
    pub(crate) config: ConfigStore,
    pub(crate) app_menu_bar: Option<Entity<AppMenuBar>>,
    pub(crate) active_title_bar_style: crate::config::TitleBarStyle,
    pub(crate) recording_action: Option<String>,
    pub(crate) active_dialog: Option<DialogKind>,
    pub(crate) renaming_saved_group: Option<String>,
    pub(crate) expanded_saved_groups: HashSet<String>,
    pub(crate) workspace_page: WorkspacePage,
    pub(crate) settings_page_open: bool,
    pub(crate) settings_page_generation: u64,
    pub(crate) settings_initial_page: usize,
    pub(crate) settings_close_remember_choice: bool,
    /// Error message when a recorded keybinding conflicts with another
    pub(crate) keybind_error: Option<(String, String)>, // (action_id, error_message)
    /// Whether workspace keybindings are currently suspended (during settings)
    pub(crate) keybinds_suspended: bool,
    pub(crate) monitoring: MonitoringState,
    pub(crate) search: SearchState,
    pub(crate) sftp_handles: std::collections::HashMap<String, crate::sftp::SftpHandle>,
    pub(crate) sftp_last_activity: HashMap<String, Instant>,
    pub(crate) runtime_state: RuntimeState,
    pub(crate) last_window_size: Option<gpui::Size<Pixels>>,
    pub(crate) last_sidebar_width: Option<Pixels>,
    pub(crate) should_move_window: bool,
    /// Detached workspace windows must not overwrite the main window's saved bounds.
    pub(crate) persist_window_layout: bool,
    /// A detached window owns one transferred terminal workspace and intentionally
    /// omits workspace navigation, SFTP, and configuration surfaces.
    pub(crate) is_detached_workspace: bool,
    pub(crate) detached_window_title: Option<String>,
    pub(crate) hovered_url: Option<HoveredUrl>,
    pub(crate) cmd_ctrl_pressed: bool,
    pub(crate) _subscriptions: Vec<gpui::Subscription>,
}
