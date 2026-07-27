use crate::config::models::InfluxConfig;
use anyhow::Result;
use influxdb3_client::{Client, ClientConfig, Point};

pub struct InfluxClient {
    client: Client,
}

impl InfluxClient {
    pub async fn new(config: &InfluxConfig) -> Result<Self> {
        let host = format!("http://{}:{}", config.host, config.port);

        let client = Client::new(
            ClientConfig::builder()
                .host(host)
                .database(&config.database)
                .token(&config.token)
                .build()?,
        )
        .await?;

        match client.ping().await {
            Ok(msg) => println!("Ping OK: {}", msg),
            Err(e) => println!("Ping failed: {}", e),
        }

        Ok(Self { client })
    }

    pub async fn test(&self) -> Result<()> {
        let point = Point::new("telemetry")
            .tag("robot", "123")
            .field("value", 42.0_f64);

        self.client.write(vec![point]).await?;

        println!("Write successful!");
        Ok(())
    }
}
