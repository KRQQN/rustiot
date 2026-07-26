use anyhow::Result;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};

use crate::config::models::MqttConfig;

pub struct MqttClient {
    client: AsyncClient,
    eventloop: EventLoop,
}

impl MqttClient {
    pub async fn new(config: &MqttConfig) -> Result<Self> {
        let mut options = MqttOptions::new(&config.client_id, &config.host, config.port);

        options.set_keep_alive(config.keep_alive);

        if let (Some(username), Some(password)) = (&config.username, &config.password) {
            options.set_credentials(username, password);
        }

        let (client, eventloop) = AsyncClient::new(options, 100);

        Ok(Self { client, eventloop })
    }

    pub async fn run(mut self) -> Result<()> {
        self.subscribe().await?;

        println!("Listening for telemetry");

        loop {
            let event = self.eventloop.poll().await?;

            self.handle_event(event).await?;
        }
    }

    async fn subscribe(&self) -> Result<()> {
        self.client
            .subscribe("devices/+/telemetry", QoS::AtLeastOnce)
            .await?;

        self.client
            .subscribe("devices/+/status", QoS::AtLeastOnce)
            .await?;

        Ok(())
    }

    async fn handle_event(&self, event: Event) -> Result<()> {
        if let Event::Incoming(Incoming::Publish(packet)) = event {
            println!("Topic: {}", packet.topic);
            println!("Payload: {:?}", packet.payload);
        }
        Ok(())
    }
}
