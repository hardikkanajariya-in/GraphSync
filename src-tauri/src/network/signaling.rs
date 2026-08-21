use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SignalingMessage {
    Offer {
        session_id: String,
        graph_id: String,
        source_device_id: String,
        target_device_id: String,
        sdp: String,
    },
    Answer {
        session_id: String,
        source_device_id: String,
        target_device_id: String,
        sdp: String,
    },
    Ice {
        session_id: String,
        source_device_id: String,
        candidate: String,
        sdp_mid: Option<String>,
        sdp_mline_index: Option<u16>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRequestMessage {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCompleteMessage {
    pub path: String,
    pub sha256: String,
    pub size: u64,
}
