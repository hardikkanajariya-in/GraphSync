use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FileOperation {
    Create,
    Update,
    Delete,
    Rename,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEvent {
    pub event_id: String,
    pub graph_id: String,
    pub device_id: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_path: Option<String>,
    pub operation: FileOperation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(default)]
    pub size: u64,
    pub revision: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingLocalChange {
    pub path: String,
    pub old_path: Option<String>,
    pub operation: FileOperation,
    pub sha256: Option<String>,
    pub size: u64,
    pub revision: u64,
}

impl SyncEvent {
    pub fn from_local(
        graph_id: &str,
        device_id: &str,
        change: PendingLocalChange,
    ) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            graph_id: graph_id.to_string(),
            device_id: device_id.to_string(),
            path: change.path,
            old_path: change.old_path,
            operation: change.operation,
            sha256: change.sha256,
            size: change.size,
            revision: change.revision,
            timestamp: Utc::now(),
        }
    }
}
