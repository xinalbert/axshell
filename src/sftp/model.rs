#[derive(Debug, Clone)]
pub struct RemoteEntry {
    pub name: String,
    pub full_path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PreviewData {
    pub path: String,
    pub title: String,
    pub body: String,
    pub is_binary: bool,
}
