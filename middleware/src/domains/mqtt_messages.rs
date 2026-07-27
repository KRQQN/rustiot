use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TelemetryMessage {
    pub msg_type: String,
    pub value: f64,
}
