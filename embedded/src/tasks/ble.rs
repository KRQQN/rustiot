use embassy_futures::join::join;
use embassy_futures::select::select;
use embassy_time::Timer;
use esp_radio::ble::controller::BleConnector;
use log::{info, warn};
use trouble_host::prelude::*;

const CONNECTIONS_MAX: usize = 1;
const L2CAP_CHANNELS_MAX: usize = 2;

#[gatt_server]
struct Server {
    battery_service: BatteryService,
}

#[gatt_service(uuid = service::BATTERY)]
struct BatteryService {
    #[descriptor(
        uuid = descriptors::VALID_RANGE,
        read,
        value = [0, 100]
    )]
    #[descriptor(
        uuid = descriptors::MEASUREMENT_DESCRIPTION,
        name = "hello",
        read,
        value = "Battery Level",
        type = &'static str
    )]
    #[characteristic(
        uuid = characteristic::BATTERY_LEVEL,
        read,
        notify,
        value = 10
    )]
    level: u8,

    #[characteristic(uuid = "408813df-5dd4-1f87-ec11-cdb001100000", read, write, notify)]
    status: (),
}

#[embassy_executor::task]
pub async fn ble_task(controller: ExternalController<BleConnector<'static>, 20>) {
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
        name: "Trouble Example",
        appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
    }))
    .unwrap();

    info!("BLE initialized");

    join(ble_runner(runner), async {
        loop {
            match advertise(&mut peripheral, &server).await {
                Ok(conn) => {
                    info!("BLE central connected");

                    let gatt = gatt_events_task(&server, &conn);
                    let custom = custom_task(&server, &conn, &stack);

                    select(gatt, custom).await;

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
            AdStructure::CompleteLocalName(b"I WANT BOOBIES"),
            AdStructure::CompleteServiceUuids16(&[[0x0f, 0x18]]),
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

    info!("BLE advertising as 'Trouble Example'");

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
    let level = server.battery_service.level;
    let status_handle = server.battery_service.status.handle;

    let mut status = false;

    loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => {
                info!("BLE disconnected: {:?}", reason);
                return Ok(());
            }

            GattConnectionEvent::Gatt { event } => {
                let reply = match event {
                    GattEvent::Read(event) => {
                        if event.handle() == level.handle {
                            let value = conn.get(&level);

                            info!("Battery level read: {:?}", value);

                            event.accept()
                        } else if event.handle() == status_handle {
                            event.accept_unprocessed(&status)
                        } else {
                            event.accept()
                        }
                    }

                    GattEvent::Write(event) => {
                        if event.handle() == status_handle {
                            match event.validate(1, 1) {
                                Ok(()) => {
                                    event.with_data(|offset, data| {
                                        if data.len() == 1 {
                                            assert_eq!(offset, 0);

                                            status = data[0] != 0;

                                            info!("Status written: {}", status);
                                        }
                                    });

                                    event.accept_unprocessed()
                                }

                                Err(err) => event.reject(err),
                            }
                        } else {
                            event.accept()
                        }
                    }

                    _ => event.accept(),
                };

                if let Ok(reply) = reply {
                    reply.send().await;
                }
            }

            _ => {}
        }
    }
}

async fn custom_task<C, P>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
    stack: &Stack<'_, C, P>,
) where
    C: Controller,
    P: PacketPool,
{
    let level = server.battery_service.level;

    let mut battery = 10u8;

    loop {
        info!("BLE notifying battery level: {}", battery);

        if level.notify(conn, &battery, true).await.is_err() {
            info!("BLE notification failed");
            return;
        }

        if let Ok(rssi) = conn.raw().rssi(stack).await {
            info!("BLE RSSI: {:?}", rssi);
        }

        battery = if battery >= 100 { 10 } else { battery + 10 };

        Timer::after_secs(2).await;
    }
}
