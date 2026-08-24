use embassy_time::{Duration, Timer};
use esp_radio::wifi::{Config as WifiConfig, WifiController, sta::StationConfig};
use log::{info, warn};

#[embassy_executor::task]
pub async fn net_task(mut runner: embassy_net::Runner<'static, esp_radio::wifi::Interface>) {
    runner.run().await;
}

#[embassy_executor::task]
pub async fn wifi_task(
    mut controller: WifiController<'static>,
    ssid: &'static str,
    password: &'static str,
) {
    let station_config = WifiConfig::Station(
        StationConfig::default()
            .with_ssid(ssid)
            .with_password(password.into()),
    );

    if let Err(e) = controller.set_config(&station_config) {
        warn!("Failed to set WiFi config: {:?}", e);
    }

    loop {
        info!("Connecting to WiFi...");

        match controller.connect_async().await {
            Ok(_) => {
                info!("WiFi connected");

                // Wait until we actually lose the connection
                match controller.wait_for_disconnect_async().await {
                    Ok(info) => warn!("WiFi disconnected: {:?}", info),
                    Err(e) => warn!("wait_for_disconnect failed: {:?}", e),
                }
            }
            Err(e) => {
                warn!("WiFi connect failed: {:?}", e);
            }
        }

        Timer::after(Duration::from_secs(5)).await;
    }
}
