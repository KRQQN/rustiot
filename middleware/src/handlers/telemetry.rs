use anyhow::Result;

use crate::domains::mqtt_messages::TelemetryMessage;

pub async fn handle(device_id: &str, payload: &[u8]) -> Result<()> {
    let message = match serde_json::from_slice::<TelemetryMessage>(payload) {
        Ok(message) => message,
        Err(err) => {
            eprintln!("Failed to parse telemetry from device: {device_id}\n error_msg: {err}");
            return Ok(());
        }
    };
    // Todo: message pipeline
    dbg!("Device id: {} \n payload: {}", device_id, message);
    Ok(())
}
