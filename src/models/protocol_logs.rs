pub enum ProtocolLogsType{
    ERROR, WARNING, INFO 
}
pub struct ProtocolLogs {
    log : String,
    logType:ProtocolLogsType
}