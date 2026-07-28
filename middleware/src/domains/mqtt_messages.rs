use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TelemetryMessage {
    pub msg_type: String,
    pub value: f64,
}

#[derive(Debug, Deserialize)]
pub struct StatusMessage {
    pub status: String,
    pub value: String,
}
