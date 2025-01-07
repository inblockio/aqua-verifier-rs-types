use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProtocolLogsType {
    ERROR,
    WARNING,
    INFO,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolLogs {
    pub log: String,
    pub log_type: ProtocolLogsType,
}
