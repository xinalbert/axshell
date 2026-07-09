use std::cmp::Ordering;

use super::super::*;

pub(super) trait SftpSortableEntry {
    fn sort_name(&self) -> &str;
    fn sort_is_dir(&self) -> bool;
    fn sort_size(&self) -> u64;
    fn sort_modified(&self) -> u32;
}

impl SftpSortableEntry for crate::sftp::RemoteEntry {
    fn sort_name(&self) -> &str {
        &self.name
    }

    fn sort_is_dir(&self) -> bool {
        self.is_dir
    }

    fn sort_size(&self) -> u64 {
        self.size
    }

    fn sort_modified(&self) -> u32 {
        self.modified
    }
}

impl SftpSortableEntry for LocalFileEntry {
    fn sort_name(&self) -> &str {
        &self.name
    }

    fn sort_is_dir(&self) -> bool {
        self.is_dir
    }

    fn sort_size(&self) -> u64 {
        self.size
    }

    fn sort_modified(&self) -> u32 {
        self.modified
    }
}

pub(super) fn sort_sftp_entries<T: SftpSortableEntry>(
    entries: &mut [T],
    column: SftpSortColumn,
    direction: SortDirection,
) {
    entries.sort_by(|a, b| {
        let dir_order = b.sort_is_dir().cmp(&a.sort_is_dir());
        if dir_order != Ordering::Equal {
            return dir_order;
        }

        let name_order = || {
            a.sort_name()
                .to_ascii_lowercase()
                .cmp(&b.sort_name().to_ascii_lowercase())
                .then_with(|| a.sort_name().cmp(b.sort_name()))
        };
        let order = match column {
            SftpSortColumn::Name => name_order(),
            SftpSortColumn::Size => a.sort_size().cmp(&b.sort_size()).then_with(name_order),
            SftpSortColumn::Modified => a
                .sort_modified()
                .cmp(&b.sort_modified())
                .then_with(name_order),
        };

        match direction {
            SortDirection::Asc => order,
            SortDirection::Desc => order.reverse(),
        }
    });
}
