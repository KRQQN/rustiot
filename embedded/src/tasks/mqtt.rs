use alloc::string::String;
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
        while !stack.is_config_up() {
            Timer::after(Duration::from_secs(1)).await;
        }

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

        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(30)));

        info!("TCP connect to {}:{}", BROKER_HOST, BROKER_PORT);
        if let Err(e) = socket.connect((ip, BROKER_PORT)).await {
            warn!("TCP connect failed: {:?}", e);
            Timer::after(Duration::from_secs(5)).await;
            continue;
        }
        info!("TCP connected");

        let mut buffer = AllocBuffer;
        let mut client: Client<'_, TcpSocket<'_>, AllocBuffer, 5, 5, 5, 5> =
            Client::new(&mut buffer);

        let client_id = match MqttString::from_str(CLIENT_ID) {
            Ok(s) => s,
            Err(_) => {
                warn!("Invalid client id");
                Timer::after(Duration::from_secs(5)).await;
                continue;
            }
        };

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

        if let Err(e) = client
            .publish(
                &PublicationOptions::new(TopicReference::Name(telem_topic.as_borrowed())),
                (&br#"{"temp":22.5,"hum":40}"#[..]).into(),
            )
            .await
        {
            warn!("Telemetry publish failed: {:?}", e);
        } else {
            info!("Published telemetry on {}", telem);
        }

        loop {
            match client.poll().await {
                Ok(event) => {
                    info!("MQTT event: {:?}", event);
                }
                Err(e) => {
                    warn!("MQTT poll error: {:?}", e);
                    break;
                }
            }
        }

        let _ = client.abort().await;
        // client + socket drop here → buffers free for next iteration
        Timer::after(Duration::from_secs(5)).await;
    }
}
