mod config;
mod domains;
mod handlers;
mod infrastructure;

use crate::{config::loader::Config, infrastructure::mqtt::MqttClient};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let config = Config::from_env()?;
    let mqtt = MqttClient::new(&config.mqtt).await?;
    mqtt.run().await?;

    Ok(())
}
