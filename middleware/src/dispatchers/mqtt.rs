use anyhow::{Ok, Result};
use rumqttc::{Event, Incoming};

use crate::{
    domains::topic::Topic,
    handlers::{status::StatusHandler, telemetry::TelemetryHandler},
};

pub struct Dispatcher {
    telemetry: TelemetryHandler,
    status: StatusHandler,
}

impl Dispatcher {
    pub fn new(telemetry: TelemetryHandler, status: StatusHandler) -> Self {
        Self { telemetry, status }
    }

    pub async fn dispatch(&self, event: Event) -> Result<()> {
        match event {
            Event::Incoming(Incoming::Publish(packet)) => match Topic::parse(&packet.topic) {
                Topic::DeviceTelemetry { device_id } => {
                    self.telemetry.handle(device_id, &packet.payload).await?;
                }

                Topic::DeviceStatus { device_id } => {
                    self.status.handle(device_id, &packet.payload).await?;
                }

                Topic::Unknown => {}
            },

            Event::Incoming(Incoming::ConnAck(_)) => {
                println!("[MQTT] Connected");
            }

            _ => {}
        }
        Ok(())
    }
}
