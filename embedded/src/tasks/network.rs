use embassy_time::{Duration, Timer};
use esp_radio::wifi::{Config as WifiConfig, WifiController, sta::StationConfig};
use log::{info, warn};

use crate::config::wifi::{WifiCredentials, load_wifi_credentials, save_wifi_credentials};

#[embassy_executor::task]
pub async fn net_task(mut runner: embassy_net::Runner<'static, esp_radio::wifi::Interface>) {
    runner.run().await;
}

#[embassy_executor::task]
pub async fn wifi_task(
    mut controller: WifiController<'static>,
    mut storage: esp_storage::FlashStorage<'static>,
) {
    info!("WiFi task started");

    let (ssid, password) = match load_wifi_credentials(&mut storage) {
        Some(creds) => {
            info!("Found stored credentials for SSID: {}", creds.ssid.as_str());
            (creds.ssid, creds.password)
        }
        None => {
            info!("No stored credentials – waiting for BLE provisioning...");
            let (ssid, password) = crate::channels::WIFI_CREDENTIALS.receive().await;
            info!("Got credentials from BLE for SSID: {}", ssid.as_str());

            let creds = WifiCredentials {
                ssid: ssid.clone(),
                password: password.clone(),
            };

            if let Err(e) = save_wifi_credentials(&mut storage, &creds) {
                warn!("Failed to save credentials to flash: {:?}", e);
            } else {
                info!("Credentials saved to flash");
            }

            (ssid, password)
        }
    };

    let station_config = WifiConfig::Station(
        StationConfig::default()
            .with_ssid(ssid.as_str())
            .with_password(password.as_str().into()),
    );

    if let Err(e) = controller.set_config(&station_config) {
        warn!("Failed to set WiFi config: {:?}", e);
        return;
    }

    loop {
        info!("Connecting to WiFi...");
        match controller.connect_async().await {
            Ok(_) => {
                info!("WiFi connected");
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
