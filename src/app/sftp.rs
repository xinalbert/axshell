use std::collections::HashSet;

use gpui::{Pixels, Point};

use crate::sftp::{PreviewData, RemoteEntry};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SftpSortColumn {
    Name,
    Size,
    Modified,
}

impl Default for SftpSortColumn {
    fn default() -> Self {
        Self::Name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SortDirection {
    Asc,
    Desc,
}

impl Default for SortDirection {
    fn default() -> Self {
        Self::Asc
    }
}

impl SortDirection {
    pub(crate) fn toggled(self) -> Self {
        match self {
            Self::Asc => Self::Desc,
            Self::Desc => Self::Asc,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SftpTransferTab {
    Active,
    Failed,
    Completed,
}

impl Default for SftpTransferTab {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Clone, Default)]
pub(crate) struct SftpUiState {
    pub(crate) current_path: String,
    pub(crate) status: String,
    pub(crate) entries: Vec<RemoteEntry>,
    pub(crate) has_more_entries: bool,
    pub(crate) loading_more_entries: bool,
    pub(crate) reached_entries_limit: bool,
    pub(crate) selected_path: Option<String>,
    pub(crate) preview: Option<PreviewData>,
    pub(crate) selected_entries: HashSet<String>,
    pub(crate) home_dir: String,
}

#[derive(Clone, Debug)]
pub(crate) struct LocalFileEntry {
    pub(crate) name: String,
    pub(crate) full_path: String,
    pub(crate) is_dir: bool,
    pub(crate) size: u64,
    pub(crate) modified: u32,
}

#[derive(Clone, Default)]
pub(crate) struct LocalFileBrowserState {
    pub(crate) current_path: String,
    pub(crate) status: String,
    pub(crate) entries: Vec<LocalFileEntry>,
    pub(crate) selected_path: Option<String>,
    pub(crate) selected_entries: HashSet<String>,
}

#[derive(Clone)]
pub(crate) enum SftpContextMenuTarget {
    Remote { path: String, is_dir: bool },
    Local { path: String, is_dir: bool },
}

#[derive(Clone)]
pub(crate) struct SftpContextMenuState {
    pub(crate) target: SftpContextMenuTarget,
    pub(crate) position: Point<Pixels>,
}
