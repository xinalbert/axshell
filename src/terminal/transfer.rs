#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TransferType {
    Upload,
    Download,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum TransferState {
    Running,
    Paused,
    Completed,
    Failed(String),
    Interrupted(String), // 中断传输：包含原因（例如 "User cancelled", "Network timeout"）
    Zombie(String),      // 程序重启后残留的 Running/Paused 任务
                         // 兼容 v0.3.11 -> v0.4.x：旧配置里曾保存过 `Cancelled`，
                         // 新版本改成了带原因的状态，因此要手动接住旧枚举值。
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
enum TransferStateCompat {
    Running,
    Paused,
    Completed,
    Failed(String),
    Interrupted(String),
    Zombie(String),
    Cancelled,
}

impl From<TransferStateCompat> for TransferState {
    fn from(value: TransferStateCompat) -> Self {
        match value {
            TransferStateCompat::Running => Self::Running,
            TransferStateCompat::Paused => Self::Paused,
            TransferStateCompat::Completed => Self::Completed,
            TransferStateCompat::Failed(reason) => Self::Failed(reason),
            TransferStateCompat::Interrupted(reason) => Self::Interrupted(reason),
            TransferStateCompat::Zombie(reason) => Self::Zombie(reason),
            TransferStateCompat::Cancelled => Self::Interrupted("Cancelled".to_string()),
        }
    }
}

impl<'de> serde::Deserialize<'de> for TransferState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        TransferStateCompat::deserialize(deserializer).map(Into::into)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransferInfo {
    pub id: String,
    pub name: String,
    pub source: String,
    pub target: String,
    pub kind: TransferType,
    pub total_bytes: Option<u64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Transfer {
    pub tab_id: String,
    pub tab_title: String,
    pub info: TransferInfo,
    pub transferred: u64,
    pub total: Option<u64>,
    pub state: TransferState,
}
