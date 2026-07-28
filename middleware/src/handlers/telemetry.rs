use crate::domains::mqtt_messages::TelemetryMessage;
use anyhow::Result;

pub struct TelemetryHandler;

impl TelemetryHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(&self, device_id: &str, payload: &[u8]) -> Result<()> {
        let Some(message) = self.parse(payload) else {
            return Ok(());
        };

        println!("Device id: {}\nPayload: {:?}", device_id, message);

        Ok(())
    }

    fn parse(&self, payload: &[u8]) -> Option<TelemetryMessage> {
        match serde_json::from_slice(payload) {
            Ok(msg) => Some(msg),
            Err(err) => {
                eprintln!("Failed to parse telemetry: {}", err);
                None
            }
        }
    }
}
