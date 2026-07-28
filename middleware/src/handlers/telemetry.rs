use std::sync::Arc;

use crate::{domains::mqtt_messages::TelemetryMessage, infrastructure::influx::InfluxClient};
use anyhow::Result;
use influxdb3_client::Point;

pub struct TelemetryHandler {
    influx: Arc<InfluxClient>,
}

impl TelemetryHandler {
    pub fn new(influx: Arc<InfluxClient>) -> Self {
        Self { influx }
    }

    pub async fn handle(&self, device_id: &str, payload: &[u8]) -> Result<()> {
        let Some(message) = self.parse(payload) else {
            return Ok(());
        };

        let point = Point::new("telemetry")
            .tag("device_id", device_id)
            .field("value", message.value);

        self.influx.write(vec![point]).await?;

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
