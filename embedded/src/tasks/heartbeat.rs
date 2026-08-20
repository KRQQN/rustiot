use embassy_time::{Duration, Timer};
use log::info;

#[embassy_executor::task]
pub async fn heartbeat_task() {
    let mut count = 0u32;
    loop {
        count += 1;
        info!("Heartbeat #{count}");
        Timer::after(Duration::from_secs(2)).await;
    }
}
