#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embedded::tasks::mqtt::mqtt_task;
use embedded::tasks::{
    ble::ble_task,
    dht11::dht11_task,
    network::{net_task, wifi_task},
};
use embedded::wifi::{self};
use embedded_dht_rs::dht11::Dht11;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{DriveMode, Flex, OutputConfig, Pull},
    timer::timg::TimerGroup,
};
use esp_radio::ble::controller::BleConnector;
use esp_storage::FlashStorage;
use log::info;
use trouble_host::prelude::ExternalController;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    info!("Booting...");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 128 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let storage = FlashStorage::new(peripherals.FLASH);

    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);
    info!("Embassy + esp-rtos started");

    let connector =
        BleConnector::new(peripherals.BT, Default::default()).expect("Failed to initialize BLE");

    let bt_controller: ExternalController<_, 20> = ExternalController::new(connector);

    spawner.spawn(ble_task(bt_controller).unwrap());

    let controller = esp_radio::wifi::WifiController::new(peripherals.WIFI, Default::default())
        .expect("Failed to initialize WiFi");

    let station = esp_radio::wifi::Interface::station();
    let wifi = wifi::create(controller, station);

    let wifi::Wifi {
        controller,
        stack,
        runner,
    } = wifi;

    spawner.spawn(net_task(runner).unwrap());
    info!("Network runner started");

    spawner.spawn(wifi_task(controller, storage).unwrap());
    info!("WiFi task started");

    while !stack.is_config_up() {
        embassy_time::Timer::after_secs(1).await;
    }
    info!("Network is up");

    spawner.spawn(mqtt_task(stack).unwrap());
    info!("MQTT task started");

    let mut dht_pin = Flex::new(peripherals.GPIO4);
    dht_pin.apply_output_config(
        &OutputConfig::default()
            .with_drive_mode(DriveMode::OpenDrain)
            .with_pull(Pull::None),
    );
    dht_pin.set_output_enable(true);
    dht_pin.set_input_enable(true);
    dht_pin.set_high();

    let delay = Delay::new();
    let dht = Dht11::new(dht_pin, delay);

    spawner.spawn(dht11_task(dht).unwrap());

    info!("Tasks spawned");
}
