use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TelemetryMessage {
    pub msg_type: String,
    pub value: f64,
}

// For now, this is a bit stupid.
#[derive(Debug, Deserialize)]
pub struct StatusMessage {
    pub msg_type: String,
    pub value: f64,
}
