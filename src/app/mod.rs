pub mod app_menu;
pub mod config_sync;
pub mod constants;
pub mod dialogs;
mod event_loop;
mod init;
pub mod keybinding_recorder;
pub mod resizable;
pub mod search;
pub mod startup;
pub mod theme;
mod types;
pub mod ui;
mod workspace;

use std::{
    collections::{HashMap, HashSet},
    sync::mpsc,
    time::Instant,
};

use crate::app::resizable::ResizableState;
use gpui::{Bounds, Entity, FocusHandle, Pixels, SharedString, UniformListScrollHandle};
use gpui_component::{ThemeMode, input::InputState};
use tokio::runtime::Runtime;

use crate::{
    session::config::{AuthMethod, ConfigStore},
    system::{SystemSampler, SystemSnapshot},
    terminal::{BackendEvent, TerminalTab},
};

pub(crate) use types::{
    ConnectionProgress, DialogKind, HoveredUrl, LocalFileBrowserState, LocalFileEntry, PaneLayout,
    SelectorEntry, SftpContextMenuState, TabGroup, TerminalFontMetrics, TerminalScrollbarHandle,
    WorkspacePage, WorkspaceTabDescriptor,
};

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
    pub(crate) global_proxy_type: String,
    pub(crate) global_proxy_host_input: Entity<InputState>,
    pub(crate) global_proxy_port_input: Entity<InputState>,
    pub(crate) global_proxy_user_input: Entity<InputState>,
    pub(crate) global_proxy_password_input: Entity<InputState>,
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
    pub(crate) custom_theme_inputs: HashMap<String, Entity<InputState>>,
    pub(crate) sync_in_progress: bool,
    pub(crate) sync_status: SharedString,
    pub(crate) sftp_path_input: Entity<InputState>,
    pub(crate) local_sftp_path_input: Entity<InputState>,
    pub(crate) ssh_auth_method: AuthMethod,
    pub(crate) editing_session_id: Option<String>,
    pub(crate) follow_system_theme: bool,
    pub(crate) theme_mode: ThemeMode,
    pub(crate) light_theme_name: SharedString,
    pub(crate) dark_theme_name: SharedString,
    pub(crate) ui_font_size: f32,
    pub(crate) terminal_font_size: f32,
    pub(crate) terminal_font_metrics: TerminalFontMetrics,
    pub(crate) terminal_zoom_accumulator: f32,
    pub(crate) ui_font_family: SharedString,
    pub(crate) terminal_font_family: SharedString,
    pub(crate) tabs: Vec<TerminalTab>,
    pub(crate) active_tab: Option<String>,
    pub(crate) tab_groups: Vec<TabGroup>,
    pub(crate) active_group: Option<String>,
    pub(crate) selector_selection: usize,
    pub(crate) workspace_panels: Entity<ResizableState>,
    pub(crate) body_panels: Entity<ResizableState>,
    pub(crate) is_layout_reset: bool,
    pub(crate) terminal_scrollbars: HashMap<String, TerminalScrollbarHandle>,
    pub(crate) remote_files_scroll_handle: UniformListScrollHandle,
    pub(crate) local_files_scroll_handle: UniformListScrollHandle,
    pub(crate) disk_scroll_handle: gpui::ScrollHandle,
    pub(crate) tabs_scroll_handle: gpui::ScrollHandle,
    pub(crate) selector_scroll_handle: gpui::ScrollHandle,
    pub(crate) saved_scroll_handle: gpui::ScrollHandle,
    pub(crate) saved_group_name_input: Entity<InputState>,
    pub(crate) connection_scroll_handle: gpui::ScrollHandle,
    pub(crate) connection_progress: Option<ConnectionProgress>,
    pub(crate) pending_sftp_path_sync: Option<String>,
    pub(crate) pending_local_sftp_path_sync: Option<String>,
    pub(crate) local_file_browser: LocalFileBrowserState,
    pub(crate) sftp_context_menu: Option<SftpContextMenuState>,
    pub(crate) sftp_creating_folder: bool,
    pub(crate) sftp_new_folder_input: Entity<InputState>,
    pub(crate) sftp_delete_scroll_handle: gpui::ScrollHandle,
    pub(crate) show_hidden_files: bool,
    pub(crate) transfers: Vec<crate::terminal::Transfer>,
    pub(crate) show_transfers_dialog: bool,
    pub(crate) system_status: Option<SharedString>,
    pub(crate) pane_root: PaneLayout,
    pub(crate) focused_pane_path: Vec<usize>,
    pub(crate) terminal_panel_bounds: Option<Bounds<Pixels>>,
    pub(crate) terminal_bounds: HashMap<String, Bounds<Pixels>>,
    pub(crate) terminal_selecting: bool,
    pub(crate) dragging_splitter: Option<(Vec<usize>, usize)>, // (parent_path, child_index)
    pub(crate) drag_split_origin: Option<gpui::Point<Pixels>>,
    pub(crate) terminal_marked_text: Option<String>,
    pub(crate) sidebar_collapsed: bool,
    pub(crate) collapsed_saved_scroll_handle: gpui::ScrollHandle,
    pub(crate) status: SharedString,
    pub(crate) config: ConfigStore,
    pub(crate) active_title_bar_style: crate::session::config::TitleBarStyle,
    pub(crate) cursor_style: crate::session::config::CursorStyle,
    pub(crate) system_sampler: SystemSampler,
    pub(crate) recording_action: Option<String>,
    pub(crate) active_dialog: Option<DialogKind>,
    pub(crate) renaming_saved_group: Option<String>,
    pub(crate) expanded_saved_groups: HashSet<String>,
    pub(crate) workspace_page: WorkspacePage,
    pub(crate) settings_page_open: bool,
    /// Error message when a recorded keybinding conflicts with another
    pub(crate) keybind_error: Option<(String, String)>, // (action_id, error_message)
    /// Whether workspace keybindings are currently suspended (during settings)
    pub(crate) keybinds_suspended: bool,
    pub(crate) system: SystemSnapshot,
    pub(crate) cpu_history: Vec<f32>,
    pub(crate) net_rx_history: Vec<f32>,
    pub(crate) net_tx_history: Vec<f32>,
    pub(crate) last_system_sample: Instant,
    pub(crate) last_theme_sync: Instant,

    pub(crate) search_input: Entity<InputState>,
    pub(crate) search_active: bool,
    pub(crate) search_query: String,
    pub(crate) search_matches: Vec<(i32, i32)>,
    pub(crate) search_current: usize,
    pub(crate) search_target_tab: Option<String>,
    pub(crate) search_bar_bounds: Option<Bounds<Pixels>>,

    pub(crate) system_tab_id: Option<String>,
    pub(crate) sftp_handles: std::collections::HashMap<String, crate::sftp::SftpHandle>,

    pub(crate) remote_sample_in_flight: bool,
    pub(crate) runtime: Runtime,
    pub(crate) events_rx: mpsc::Receiver<BackendEvent>,
    pub(crate) events_tx: mpsc::Sender<BackendEvent>,
    pub(crate) last_window_size: Option<gpui::Size<Pixels>>,
    pub(crate) last_sidebar_width: Option<Pixels>,
    pub(crate) should_move_window: bool,
    pub(crate) hovered_url: Option<HoveredUrl>,
    pub(crate) cmd_ctrl_pressed: bool,
    pub(crate) pending_terminal_refresh: bool,
    pub(crate) last_terminal_refresh: Instant,
    pub(crate) _subscriptions: Vec<gpui::Subscription>,
}
