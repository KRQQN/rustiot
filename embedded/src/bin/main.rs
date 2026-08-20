#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_dht_rs::dht11::Dht11;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{DriveMode, Flex, Level, Output, OutputConfig, Pull},
    interrupt::software::SoftwareInterruptControl,
    timer::timg::TimerGroup,
};
use log::info;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    // ESP_LOG="debug || info || warn"
    // --- target specific crate ---
    // ESP_LOG=info, embedded::led_task=debug
    esp_println::logger::init_logger_from_env();
    info!("Booting...");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);
    info!("Embassy + esp-rtos started");

    let mut dht_pin = Flex::new(peripherals.GPIO4);

    dht_pin.apply_output_config(
        &OutputConfig::default()
            .with_drive_mode(DriveMode::OpenDrain)
            .with_pull(Pull::None),
    );
    dht_pin.set_output_enable(true);
    dht_pin.set_input_enable(true);
    dht_pin.set_high(); // idle state

    let delay = Delay::new();
    let dht = Dht11::new(dht_pin, delay);

    let led = Output::new(peripherals.GPIO48, Level::High, OutputConfig::default());
    spawner.spawn(led_task(led).unwrap());
    spawner.spawn(heartbeat_task().unwrap());
    spawner.spawn(dht11_task(dht).unwrap());

    info!("Tasks spawned");
}

#[embassy_executor::task]
async fn led_task(mut led: Output<'static>) {
    loop {
        led.toggle();
        info!("LED toggled");
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn heartbeat_task() {
    let mut count = 0u32;
    loop {
        count += 1;
        info!("Heartbeat #{count}");
        Timer::after(Duration::from_secs(2)).await;
    }
}

#[embassy_executor::task]
async fn dht11_task(mut dht: Dht11<Flex<'static>, Delay>) {
    // Give the sensor a moment after power-up
    Timer::after(Duration::from_secs(2)).await;

    loop {
        match dht.read() {
            Ok(reading) => {
                info!(
                    "DHT11 → Temperature: {:.1} °C | Humidity: {:.1} %",
                    reading.temperature, reading.humidity
                );
            }
            Err(e) => {
                info!("DHT11 read error: {:?}", e);
            }
        }

        // DHT11 needs ~1–2s between readings
        Timer::after(Duration::from_secs(5)).await;
    }
}
