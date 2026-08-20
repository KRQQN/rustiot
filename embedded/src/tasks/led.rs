use embassy_time::{Duration, Timer};
use esp_hal::gpio::Output;
use log::info;

#[embassy_executor::task]
pub async fn led_task(mut led: Output<'static>) {
    loop {
        led.toggle();
        info!("LED toggled");
        Timer::after(Duration::from_millis(500)).await;
    }
}
