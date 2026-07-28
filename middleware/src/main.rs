mod config;
mod dispatchers;
mod domains;
mod handlers;
mod infrastructure;

use std::sync::Arc;

use crate::{
    config::loader::Config,
    dispatchers::mqtt::Dispatcher,
    handlers::{status::StatusHandler, telemetry::TelemetryHandler},
    infrastructure::{influx::InfluxClient, mqtt::MqttClient, redis::RedisClient},
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;

    let influx = Arc::new(InfluxClient::new(&config.influx).await?);
    let redis = Arc::new(RedisClient::new("redis://127.0.0.1")?);
    let dispatcher = Dispatcher::new(
        TelemetryHandler::new(influx.clone()),
        StatusHandler::new(redis.clone()),
    );
    let mqtt = MqttClient::new(&config.mqtt, dispatcher).await?;

    mqtt.run().await?;

    Ok(())
}
