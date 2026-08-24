use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embedded_dht_rs::SensorReading;

pub static SENSOR_CHANNEL: Channel<CriticalSectionRawMutex, SensorReading<u8>, 1> = Channel::new();
pub static WIFI_CREDENTIALS: Channel<
    CriticalSectionRawMutex,
    (heapless::String<32>, heapless::String<64>),
    1,
> = Channel::new();
