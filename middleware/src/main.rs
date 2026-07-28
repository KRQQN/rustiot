mod config;
mod dispatchers;
mod domains;
mod handlers;
mod infrastructure;

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

    let influx = InfluxClient::new(&config.influx).await?;
    let dispatcher = Dispatcher::new(TelemetryHandler::new(), StatusHandler::new());
    let mqtt = MqttClient::new(&config.mqtt, dispatcher).await?;

    influx.test().await?;
    mqtt.run().await?;

    Ok(())
}
