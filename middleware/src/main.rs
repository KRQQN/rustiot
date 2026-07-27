mod config;
mod domains;
mod handlers;
mod infrastructure;

use crate::{
    config::loader::Config,
    infrastructure::{influx::InfluxClient, mqtt::MqttClient},
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let config = Config::from_env()?;
    let influx = InfluxClient::new(&config.influx).await?;
    let mqtt = MqttClient::new(&config.mqtt).await?;
    influx.test().await?;
    mqtt.run().await?;

    Ok(())
}
