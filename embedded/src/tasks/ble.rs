use embassy_futures::join::join;
use embassy_time::Timer;
use heapless::String;
use log::{info, warn};
use trouble_host::prelude::*;

const CONNECTIONS_MAX: usize = 1;
const L2CAP_CHANNELS_MAX: usize = 2;

const STATUS_CREDENTIALS_RECEIVED: u8 = 1;
const STATUS_CHANNEL_FULL: u8 = 2;

#[gatt_server]
struct Server {
    wifi_service: WifiProvisionService,
}

#[gatt_service(uuid = "12345678-1234-5678-1234-56789abcdef0")]
struct WifiProvisionService {
    #[characteristic(
        uuid = "12345678-1234-5678-1234-56789abcdef1",
        write,
        write_without_response
    )]
    ssid: u8,

    #[characteristic(
        uuid = "12345678-1234-5678-1234-56789abcdef2",
        write,
        write_without_response
    )]
    password: u8,

    #[characteristic(
        uuid = "12345678-1234-5678-1234-56789abcdef3",
        read,
        notify,
        value = 0u8
    )]
    status: u8,
}

#[embassy_executor::task]
pub async fn ble_task(
    controller: ExternalController<esp_radio::ble::controller::BleConnector<'static>, 20>,
) {
    run(controller).await;
}

async fn run<C>(controller: C)
where
    C: Controller,
{
    let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> =
        HostResources::new();

    let stack = trouble_host::new(controller, &mut resources).build();

    let runner = stack.runner();
    let mut peripheral = stack.peripheral();

    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: "WiFi Provision",
        appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
    }))
    .unwrap();

    info!("BLE initialized");

    join(ble_runner(runner), async {
        loop {
            match advertise(&mut peripheral, &server).await {
                Ok(conn) => {
                    info!("BLE central connected");

                    let _ = gatt_events_task(&server, &conn).await;

                    info!("BLE connection closed");
                }

                Err(e) => {
                    warn!("BLE advertising error: {:?}", e);

                    Timer::after_millis(100).await;
                }
            }
        }
    })
    .await;
}

async fn ble_runner<C, P>(mut runner: Runner<'_, C, P>)
where
    C: Controller,
    P: PacketPool,
{
    loop {
        if let Err(e) = runner.run().await {
            warn!("BLE runner error: {:?}", e);
            Timer::after_millis(100).await;
        }
    }
}

async fn advertise<'values, 'server, C>(
    peripheral: &mut Peripheral<'values, C, DefaultPacketPool>,
    server: &'server Server<'values>,
) -> Result<GattConnection<'values, 'server, DefaultPacketPool>, BleHostError<C::Error>>
where
    C: Controller,
{
    let mut advertiser_data = [0u8; 31];

    let len = AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::CompleteLocalName(b"WiFi Provision"),
        ],
        &mut advertiser_data,
    )?;

    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertiser_data[..len],
                scan_data: &[],
            },
        )
        .await?;

    info!("BLE advertising as 'WiFi Provision'");

    let conn = advertiser.accept().await?.with_attribute_server(server)?;

    info!("BLE connection established");

    Ok(conn)
}

async fn gatt_events_task<P>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
) -> Result<(), Error>
where
    P: PacketPool,
{
    let ssid_handle = server.wifi_service.ssid.handle;
    let password_handle = server.wifi_service.password.handle;
    let status = server.wifi_service.status;

    let mut pending_ssid: Option<String<32>> = None;
    let mut pending_password: Option<String<64>> = None;

    info!("Waiting for WiFi credentials");

    loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => {
                info!("BLE disconnected: {:?}", reason);

                return Ok(());
            }

            GattConnectionEvent::Gatt { event } => {
                let reply = match event {
                    GattEvent::Write(event) => {
                        if event.handle() == ssid_handle {
                            match event.validate(1, 32) {
                                Ok(()) => {
                                    event.with_data(|offset, data| {
                                        if offset != 0 {
                                            warn!(
                                                "SSID write has \
                                                     non-zero offset: {}",
                                                offset
                                            );
                                            return;
                                        }

                                        match core::str::from_utf8(data) {
                                            Ok(value) => {
                                                let mut ssid = String::<32>::new();

                                                if ssid.push_str(value).is_ok() {
                                                    pending_ssid = Some(ssid);

                                                    info!("Received SSID");
                                                } else {
                                                    warn!("SSID too long");
                                                }
                                            }

                                            Err(_) => {
                                                warn!(
                                                    "SSID is not \
                                                         valid UTF-8"
                                                );
                                            }
                                        }
                                    });

                                    event.accept()
                                }

                                Err(e) => event.reject(e),
                            }
                        } else if event.handle() == password_handle {
                            match event.validate(1, 64) {
                                Ok(()) => {
                                    event.with_data(|offset, data| {
                                        if offset != 0 {
                                            warn!(
                                                "Password write has \
                                                     non-zero offset: {}",
                                                offset
                                            );
                                            return;
                                        }

                                        match core::str::from_utf8(data) {
                                            Ok(value) => {
                                                let mut password = String::<64>::new();

                                                if password.push_str(value).is_ok() {
                                                    pending_password = Some(password);

                                                    info!("Received password");
                                                } else {
                                                    warn!("Password too long");
                                                }
                                            }

                                            Err(_) => {
                                                warn!(
                                                    "Password is not \
                                                         valid UTF-8"
                                                );
                                            }
                                        }
                                    });

                                    event.accept()
                                }

                                Err(e) => event.reject(e),
                            }
                        } else {
                            event.accept()
                        }
                    }

                    GattEvent::Read(event) => {
                        if event.handle() == status.handle {
                            event.accept()
                        } else {
                            event.accept()
                        }
                    }

                    _ => event.accept(),
                };

                if let Ok(reply) = reply {
                    reply.send().await;
                }

                info!(
                    "Credential state: ssid={}, password={}",
                    pending_ssid.is_some(),
                    pending_password.is_some()
                );

                if pending_ssid.is_some() && pending_password.is_some() {
                    info!("Credentials complete");

                    let ssid = pending_ssid.take().unwrap();

                    let password = pending_password.take().unwrap();

                    info!("SSID: {}", ssid.as_str());

                    info!("Password length: {}", password.len());

                    match crate::channels::WIFI_CREDENTIALS.try_send((ssid, password)) {
                        Ok(()) => {
                            info!("Credentials handed to WiFi task");

                            if status
                                .notify(conn, &STATUS_CREDENTIALS_RECEIVED, true)
                                .await
                                .is_err()
                            {
                                warn!("Failed to notify status");
                            }
                        }

                        Err(_) => {
                            warn!("WIFI_CREDENTIALS channel full");

                            if status
                                .notify(conn, &STATUS_CHANNEL_FULL, true)
                                .await
                                .is_err()
                            {
                                warn!("Failed to notify status");
                            }
                        }
                    }
                }
            }

            _ => {}
        }
    }
}
