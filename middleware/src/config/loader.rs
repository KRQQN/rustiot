use std::{env, time::Duration};

use crate::config::models::MqttConfig;

pub struct Config {
    pub mqtt: MqttConfig,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let mqtt = MqttConfig {
            host: env::var("MQTT_HOST")?,
            port: env::var("MQTT_PORT")?.parse()?,
            client_id: env::var("MQTT_CLIENT_ID")?,
            username: env::var("MQTT_USERNAME").ok(),
            password: env::var("MQTT_PASSWORD").ok(),
            keep_alive: Duration::from_secs(
                env::var("MQTT_KEEP_ALIVE")
                    .unwrap_or_else(|_| "30".into())
                    .parse()?,
            ),
        };

        Ok(Self { mqtt })
    }
}
