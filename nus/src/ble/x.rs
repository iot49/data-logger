use defmt::*;

use nrf_softdevice::{raw, Softdevice};
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{CharacteristicHandles, RegisterError, WriteOp, Service};
use nrf_softdevice::ble::{gatt_server, peripheral, Connection, Uuid};

use embassy_executor::Spawner;
use embassy_time::{Timer, Duration};

use futures::future::{select, Either};
use futures::pin_mut;
use uuid::uuid;
use array_concat::concat_arrays;

use super::battery_service::BatteryService;
use super::nus_service::{NusService, NusData};
use super::config::{softdevice_config, softdevice_task};

const NAME: &[u8; 8] = b"Nus Test";

const NUS_SV_UUID: &[u8; 16] = &uuid!("6e400001-b5a3-f393-e0a9-e50e24dcca9e").to_u128_le().to_be_bytes();


struct Server {
    bas: BatteryService,
    nus: NusService,
}

impl Server {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let bas = BatteryService::new(sd)?;
        let nus: NusService = NusService::new(sd)?;
        Ok(Self { bas, nus })
    }
}

impl gatt_server::Server for Server {
    type Event = ();

    fn on_write(
        &self,
        _conn: &Connection,
        handle: u16,
        _op: WriteOp,
        _offset: usize,
        data: &[u8],
    ) -> Option<Self::Event> {
        info!("--- gatt_server::on_write op={} offset={} data={}", _op, _offset, data.len());
        self.nus.on_write(handle, data);
        self.bas.on_write(handle, data);
        None
    }
}


#[embassy_executor::task]
pub async fn main_task() {
    #[rustfmt::skip]
    let adv_data: &[u8; 3+18+2+NAME.len()] = &concat_arrays!(
        [ 2u8, raw::BLE_GAP_AD_TYPE_FLAGS as u8, raw::BLE_GAP_ADV_FLAGS_LE_ONLY_GENERAL_DISC_MODE as u8, ],
        [ (NUS_SV_UUID.len()+1) as u8, raw::BLE_GAP_AD_TYPE_128BIT_SERVICE_UUID_COMPLETE as u8 ], *NUS_SV_UUID,
        [ (NAME.len() + 1) as u8, raw::BLE_GAP_AD_TYPE_COMPLETE_LOCAL_NAME as u8 ], *NAME
    );
    let scan_data = &[];

    let sd = Softdevice::enable(&softdevice_config(NAME));
    let server = unwrap!(Server::new(sd));
    let spawner: Spawner = Spawner::for_current_executor().await;
    unwrap!(spawner.spawn(softdevice_task(sd)));

    loop {
        info!("Advertise ...");
        let config = peripheral::Config::default();
        let adv = peripheral::ConnectableAdvertisement::ScannableUndirected { adv_data, scan_data };
        let conn = unwrap!(peripheral::advertise_connectable(sd, adv, &config).await);

        info!("Connecting ...");

        // let data_fut = bat_fut(sd, &server, &conn);
        let data_fut = nus_fut(&server, &conn);
        let gatt_fut = gatt_server::run(&conn, &server, |e| info!("gatt_server::run {}", e));

        info!("Connected ...");
        pin_mut!(data_fut);
        pin_mut!(gatt_fut);

        let _ = match select(data_fut, gatt_fut).await {
            Either::Left((_, _)) => {
                info!("data_fut exited")
            }
            Either::Right((res, _)) => {
                if let Err(e) = res {
                    info!("gatt_server run exited with error: {:?}", e);
                }
            }
        };

        info!("Disconnected.");

    }
}

async fn nus_fut<'a>(server: &'a Server, connection: &'a Connection) {
    for i in 0..255 {
        let c = b'a' + (i%26);
        let mut val = NusData::from_slice(b"msg 0123456789 0123456789 0123456789").unwrap();
        // let mut val = NusData::from_slice(b"msg ").unwrap();
        val.push(c);
        
        match server.nus.tx_notify(connection, &val) {
            Ok(_) => (),
            Err(_) => {
                // info!("nus.tx_notify failed for {} - client has notifications disabled?", c);
            }
        }
        Timer::after(Duration::from_secs(1)).await
    }
}

async fn bat_fut<'a>(sd: &'a Softdevice, server: &'a Server, connection: &'a Connection) {
    for i in 0..100 {
        let batt_raw_value: u8 = i;

        match server.bas.battery_level_notify(connection, batt_raw_value) {
            Ok(_) => info!("Battery raw_value: {}", batt_raw_value),
            Err(_) => {
                info!("Battery set {}", batt_raw_value);
                unwrap!(server.bas.battery_level_set(sd, batt_raw_value));
            }
        };

        Timer::after(Duration::from_secs(1)).await
    }
}

