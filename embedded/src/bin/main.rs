#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
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

    let led = Output::new(peripherals.GPIO48, Level::High, OutputConfig::default());

    spawner.spawn(led_task(led).unwrap());
    spawner.spawn(heartbeat_task().unwrap());

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
