use std::{env, time::Duration};

use crate::config::models::{InfluxConfig, MqttConfig};

pub struct Config {
    pub mqtt: MqttConfig,
    pub influx: InfluxConfig,
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

        let influx = InfluxConfig {
            host: env::var("INFLUX_HOST")?,
            port: env::var("INFLUX_PORT")?.parse()?,
            database: env::var("INFLUX_DATABASE")?,
            token: super::influx_token::load_influx_token()?,
        };

        Ok(Self { mqtt, influx })
    }
}
