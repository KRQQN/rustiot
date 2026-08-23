use crate::channels::SENSOR_CHANNEL;

use alloc::{format, string::String};
use embassy_futures::select::{Either, select};
use embassy_net::{Stack, dns::DnsQueryType, tcp::TcpSocket};
use embassy_time::{Duration, Timer};
use log::{info, warn};

use rust_mqtt::{
    buffer::AllocBuffer,
    client::{
        Client,
        options::{ConnectOptions, PublicationOptions, TopicReference},
    },
    config::SessionExpiryInterval,
    types::{MqttString, TopicName},
};

const BROKER_HOST: &str = "192.168.1.95";
const BROKER_PORT: u16 = 1883;
const CLIENT_ID: &str = "esp-rustiot";
const DEVICE_ID: &str = "esp-rustiot";

fn telemetry_topic() -> String {
    let mut s = String::new();
    s.push_str("devices/");
    s.push_str(DEVICE_ID);
    s.push_str("/telemetry");
    s
}

fn status_topic() -> String {
    let mut s = String::new();
    s.push_str("devices/");
    s.push_str(DEVICE_ID);
    s.push_str("/status");
    s
}

#[embassy_executor::task]
pub async fn mqtt_task(stack: Stack<'static>) {
    let mut rx_buffer = [0u8; 4096];
    let mut tx_buffer = [0u8; 4096];

    let connect_options = ConnectOptions::new()
        .clean_start()
        .session_expiry_interval(SessionExpiryInterval::NeverEnd);

    loop {
        // Wait for the network to have an IP configuration.
        while !stack.is_config_up() {
            Timer::after(Duration::from_secs(1)).await;
        }

        // Resolve the MQTT broker address.
        let ip = match stack.dns_query(BROKER_HOST, DnsQueryType::A).await {
            Ok(addrs) if !addrs.is_empty() => addrs[0],

            Ok(_) => {
                warn!("DNS returned no addresses");
                Timer::after(Duration::from_secs(5)).await;
                continue;
            }

            Err(e) => {
                warn!("DNS error: {:?}", e);
                Timer::after(Duration::from_secs(5)).await;
                continue;
            }
        };

        // Create TCP socket.
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(30)));
        info!("TCP connecting to {}:{}", BROKER_HOST, BROKER_PORT);

        if let Err(e) = socket.connect((ip, BROKER_PORT)).await {
            warn!("TCP connect failed: {:?}", e);
            Timer::after(Duration::from_secs(5)).await;
            continue;
        }
        info!("TCP connected");

        // Create MQTT client.
        let mut buffer = AllocBuffer;

        let mut client: Client<'_, TcpSocket<'_>, AllocBuffer, 5, 5, 5, 5> =
            Client::new(&mut buffer);

        // MQTT client ID.
        let client_id = match MqttString::from_str(CLIENT_ID) {
            Ok(s) => s,

            Err(_) => {
                warn!("Invalid client id");
                Timer::after(Duration::from_secs(5)).await;
                continue;
            }
        };

        // Connect MQTT.
        if let Err(e) = client
            .connect(socket, &connect_options, Some(client_id))
            .await
        {
            warn!("MQTT connect failed: {:?}", e);
            Timer::after(Duration::from_secs(5)).await;
            continue;
        }
        info!("MQTT connected");

        let telem = telemetry_topic();
        let status = status_topic();

        let Some(telem_topic) = TopicName::new(MqttString::from_str(telem.as_str()).unwrap())
        else {
            warn!("Invalid telemetry topic");
            let _ = client.abort().await;
            Timer::after(Duration::from_secs(5)).await;
            continue;
        };

        let Some(status_topic_name) =
            TopicName::new(MqttString::from_str(status.as_str()).unwrap())
        else {
            warn!("Invalid status topic");
            let _ = client.abort().await;
            Timer::after(Duration::from_secs(5)).await;
            continue;
        };

        if let Err(e) = client
            .publish(
                &PublicationOptions::new(TopicReference::Name(status_topic_name.as_borrowed())),
                (&b"online"[..]).into(),
            )
            .await
        {
            warn!("Status publish failed: {:?}", e);
        } else {
            info!("Published status=online on {}", status);
        }

        // * Alternative to solution implemented in tha loop would be timeout with embassy_time
        // match embassy_time::with_timeout(POLL_INTERVAL, SENSOR_CHANNEL.receive()).await {
        //     Ok(reading) => {}
        //     Err(_) => {}
        // }
        loop {
            match select(
                // Wait for whichever happens first:
                SENSOR_CHANNEL.receive(), //  1. An DHT11 reading
                client.poll()).await      //  2. An MQTT event
            {
                Either::First(reading) => {
                    info!(
                        "Received DHT11: {} °C, {} %",
                        reading.temperature, reading.humidity
                    );

                    let payload = format!(
                        r#"{{"temp":{},"hum":{}}}"#,
                        reading.temperature, reading.humidity
                    );

                    if let Err(e) = client
                        .publish(
                            &PublicationOptions::new(TopicReference::Name(
                                telem_topic.as_borrowed(),
                            )),
                            payload.as_bytes().into(),
                        )
                        .await
                    {
                        warn!("Telemetry publish failed: {:?}", e);
                        break;
                    }

                    info!("Published telemetry on {}", telem);
                }

                Either::Second(result) => match result {
                    Ok(event) => {
                        info!("MQTT event: {:?}", event);
                    }

                    Err(e) => {
                        warn!("MQTT poll error: {:?}", e);
                        break;
                    }
                },
            }
        }

        info!("MQTT connection closed");
        let _ = client.abort().await;
        Timer::after(Duration::from_secs(5)).await;
    }
}
