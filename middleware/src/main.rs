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
    infrastructure::{influx::InfluxClient, mqtt::MqttClient},
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;

    let influx = Arc::new(InfluxClient::new(&config.influx).await?);
    let dispatcher = Dispatcher::new(TelemetryHandler::new(influx.clone()), StatusHandler::new());
    let mqtt = MqttClient::new(&config.mqtt, dispatcher).await?;

    mqtt.run().await?;

    Ok(())
}
