use std::sync::Arc;

use crate::{domains::mqtt_messages::StatusMessage, infrastructure::redis::RedisClient};
use anyhow::Result;

pub struct StatusHandler {
    redis: Arc<RedisClient>,
}

impl StatusHandler {
    pub fn new(redis: Arc<RedisClient>) -> Self {
        Self { redis }
    }

    pub async fn handle(&self, device_id: &str, payload: &[u8]) -> Result<()> {
        let Some(message) = self.parse(payload) else {
            return Ok(());
        };

        println!("Device id: {}\nPayload: {:?}", device_id, message);

        Ok(())
    }

    fn parse(&self, payload: &[u8]) -> Option<StatusMessage> {
        match serde_json::from_slice(payload) {
            Ok(msg) => Some(msg),
            Err(err) => {
                eprintln!("Failed to parse status: {}", err);
                None
            }
        }
    }
}
