use embassy_net::{Config as NetConfig, Runner, Stack, StackResources};
use esp_radio::wifi::{Config as WifiConfig, Interface, WifiController, sta::StationConfig};
use static_cell::StaticCell;

pub struct Wifi {
    pub controller: WifiController<'static>,
    pub stack: Stack<'static>,
    pub runner: Runner<'static, Interface<'static>>,
}

pub fn create(controller: WifiController<'static>, station: Interface<'static>) -> Wifi {
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();

    let resources = RESOURCES.init(StackResources::new());

    let net_config = NetConfig::dhcpv4(Default::default());
    let seed = 0x1234_5678_u64;

    let (stack, runner) = embassy_net::new(station, net_config, resources, seed);

    Wifi {
        controller,
        stack,
        runner,
    }
}

pub async fn connect(
    controller: &mut WifiController<'static>,
    ssid: &'static str,
    password: &'static str,
) {
    let station_config = WifiConfig::Station(
        StationConfig::default()
            .with_ssid(ssid)
            .with_password(password.into()),
    );

    controller
        .set_config(&station_config)
        .expect("Failed to configure WiFi");

    controller
        .connect_async()
        .await
        .expect("Failed to connect to WiFi");
}
