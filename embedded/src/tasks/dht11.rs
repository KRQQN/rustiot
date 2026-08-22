use crate::channels::SENSOR_CHANNEL;
use embassy_time::{Duration, Timer};
use embedded_dht_rs::dht11::Dht11;
use esp_hal::{delay::Delay, gpio::Flex};
use log::info;

#[embassy_executor::task]
pub async fn dht11_task(mut dht: Dht11<Flex<'static>, Delay>) {
    // Give the sensor a moment after power-up
    Timer::after(Duration::from_secs(2)).await;
    loop {
        match dht.read() {
            Ok(reading) => {
                info!(
                    "DHT11 → Temperature: {:.1} °C | Humidity: {:.1} %",
                    reading.temperature, reading.humidity
                );
                SENSOR_CHANNEL.send(reading).await;
            }
            Err(e) => {
                info!("DHT11 read error: {:?}", e);
            }
        }

        // DHT11 needs ~1–2s between readings
        Timer::after(Duration::from_secs(5)).await;
    }
}
