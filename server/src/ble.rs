/// Setup and start bluetooth:
/// 1. Softdevice (ble::Config)
/// 2. Advertisement
/// 3. Peripheral Server (NusService)
/// 4. Scanner

use defmt::*;

use nrf_softdevice::{raw, Softdevice};
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{CharacteristicHandles, RegisterError, WriteOp, Service};
use nrf_softdevice::ble::{gatt_server, peripheral, Connection, Uuid};

use embassy_executor::Spawner;

use futures::future::{select, Either};
use futures::pin_mut;
use uuid::uuid;
use array_concat::concat_arrays;

mod config;
mod scanner;
mod nus_service;

use config::*;
use scanner::*;
use nus_service::*;

use super::comm::Comm;

struct Server {
    nus: NusService,
}

impl Server {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let nus: NusService = NusService::new(sd)?;
        Ok(Self { nus })
    }
}

impl gatt_server::Server for Server {
    type Event = ();

    /// receiver
    fn on_write(
        &self,
        _conn: &Connection,
        handle: u16,
        _op: WriteOp,
        _offset: usize,
        data: &[u8],
    ) -> Option<Self::Event> {
        debug!("--- gatt_server::on_write op={} offset={} data={}", _op, _offset, data.len());
        self.nus.on_write(handle, data);
       None
    }
}


#[embassy_executor::task]
pub async fn main_task(comm: &'static Comm) {
    #[rustfmt::skip]
    let adv_data: &[u8; 3+18+2+NAME.len()] = &concat_arrays!(
        [ 2u8, raw::BLE_GAP_AD_TYPE_FLAGS as u8, raw::BLE_GAP_ADV_FLAGS_LE_ONLY_GENERAL_DISC_MODE as u8, ],
        [ (NUS_SV_UUID.len()+1) as u8, raw::BLE_GAP_AD_TYPE_128BIT_SERVICE_UUID_COMPLETE as u8 ], *NUS_SV_UUID,
        [ (NAME.len() + 1) as u8, raw::BLE_GAP_AD_TYPE_COMPLETE_LOCAL_NAME as u8 ], *NAME
    );
    let scan_data = &[];

    // Note: start server before spawning softdevice_task to avoid borrow conflicts
    let sd = Softdevice::enable(&softdevice_config());
    let server = unwrap!(Server::new(sd));
    let spawner: Spawner = Spawner::for_current_executor().await;
    unwrap!(spawner.spawn(softdevice_task(sd)));

    // scanner
    unwrap!(spawner.spawn(scanner::main_task(sd, comm)));

    // peripheral
    loop {
        info!("Advertise ...");
        let config = peripheral::Config::default();
        let adv = peripheral::ConnectableAdvertisement::ScannableUndirected { adv_data, scan_data };
        let conn = unwrap!(peripheral::advertise_connectable(sd, adv, &config).await);

        info!("Connecting ...");

        // let data_fut = bat_fut(sd, &server, &conn);
        let data_fut = nus_fut(comm, &server.nus, &conn);
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


#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) -> ! {
    debug!("start softdevice");
    sd.run().await
}
