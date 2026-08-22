use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embedded_dht_rs::SensorReading;

pub static SENSOR_CHANNEL: Channel<CriticalSectionRawMutex, SensorReading<u8>, 1> = Channel::new();
