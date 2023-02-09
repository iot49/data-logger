#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

#[path = "../example_common.rs"]
mod example_common;

use core::mem;

use array_concat::concat_arrays;
use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use futures::future::{select, Either};
use futures::pin_mut;
use heapless::Vec as HeaplessVec;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{NotifyValueError, RegisterError, Service, WriteOp};
use nrf_softdevice::ble::{gatt_server, peripheral, Connection, GattValue, Uuid};
use nrf_softdevice::{raw, Softdevice};
use uuid::uuid;

const NUS_MAX_LEN: usize = 20;

type NusData = HeaplessVec<u8, NUS_MAX_LEN>;

/// Service Handles
pub struct NusService {
    rx_valh: u16,
    tx_valh: u16,
    tx_cccd: u16,
}

// #[nrf_softdevice::gatt_service(uuid = )]
// pub struct NusService {
//     #[characteristic(uuid = "6e400002-b5a3-f393-e0a9-e50e24dcca9e", write, write_without_response)]
//     rx: NusData,
//     #[characteristic(uuid = "6e400003-b5a3-f393-e0a9-e50e24dcca9e", notify)]
//     tx: NusData,
// }

// Note: this combination of uuid::uuid! macro plus the const functions does
// manage to convert the string representation directly to a [u8; 16] that
// is stored in .rodata (flash) but doing it this way does mean that each
// of these is using 16 bytes.
// This was confirmed via cargo objdump --target thumbv7em-none-eabihf -- -s -S | less
// We could have code that does it more efficiently, either by copying
// the base UUID to RAM and making changes to bytes 12 and 13 there
// before calling new_128(), or by using Uuid::from_raw_parts()
// to get the type field from the new_128() call and using it to directly make
// Uuids from the 16-bit values.
const NUS_SVC_UUID: &[u8; 16] = &uuid!("6e400001-b5a3-f393-e0a9-e50e24dcca9e").to_u128_le().to_be_bytes();
const NUS_RX_UUID: &[u8; 16] = &uuid!("6e400002-b5a3-f393-e0a9-e50e24dcca9e").to_u128_le().to_be_bytes();
const NUS_TX_UUID: &[u8; 16] = &uuid!("6e400003-b5a3-f393-e0a9-e50e24dcca9e").to_u128_le().to_be_bytes();

impl NusService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let uuid = Uuid::new_128(NUS_SVC_UUID);
        info!("nus svc {:?}", unsafe { *uuid.as_raw_ptr() }.type_);
        let mut svc = ServiceBuilder::new(sd, uuid)?;

        let attr = Attribute::new(&[0u8])
            // .write_security(SecurityMode::JustWorks)
            // .deferred_write()
            .variable_len(NUS_MAX_LEN as u16);
        let props = Properties::new().write().write_without_response();

        let uuid = Uuid::new_128(NUS_RX_UUID);
        info!("nus rx {:?}", unsafe { *uuid.as_raw_ptr() }.type_);
        let builder = svc.add_characteristic(uuid, attr, Metadata::new(props))?;
        let c_rx = builder.build();

        let attr = Attribute::new(&[0u8]).variable_len(NUS_MAX_LEN as u16);
        let props = Properties::new().notify();
        let uuid = Uuid::new_128(NUS_TX_UUID);
        info!("nus tx {:?}", unsafe { *uuid.as_raw_ptr() }.type_);
        let builder = svc.add_characteristic(uuid, attr, Metadata::new(props))?;
        let c_tx = builder.build();

        let _service_handle = svc.build();

        Ok(NusService {
            rx_valh: c_rx.value_handle,
            tx_valh: c_tx.value_handle,
            tx_cccd: c_tx.cccd_handle,
        })
    }

    pub fn tx_notify(&self, conn: &Connection, val: &NusData) -> Result<(), NotifyValueError> {
        let buf = GattValue::to_gatt(*&val);
        info!("A tx_notify h=0x{:04x} d=0x{:02x}", self.tx_valh, buf);
        info!("B tx_notify h=0x{:04x} d=0x{:02x}", self.rx_valh, buf);
        //gatt_server::notify_value(conn, self.tx_valh, buf)
        gatt_server::notify_value(conn, self.rx_valh, buf)
    }

    pub fn tx_set(&self, sd: &Softdevice, val: &NusData) {
        let buf = GattValue::to_gatt(*&val);
        info!("A tx_set h=0x{:04x} d=0x{:02x}", self.rx_valh, buf);
        gatt_server::set_value(sd, self.rx_valh, val);
    }
}

impl Service for NusService {
    type Event = NusServiceEvent;

    fn on_write(&self, handle: u16, data: &[u8]) -> Option<Self::Event> {
        // data received from central
        if let Ok(s) = core::str::from_utf8(&data) {
            info!("on_write str {}", s);
        } else {
            info!("on_write [u8] {}", data);
        }
        if handle == self.rx_valh {
            info!("rx_valh {}", self.rx_valh);
            return Some(NusServiceEvent::RxWrite(
                <NusData as ::nrf_softdevice::ble::GattValue>::from_gatt(data),
            ));
        }
        if handle == self.tx_cccd && !data.is_empty() {
            info!("tx_cccd {}, data = {}", self.tx_cccd, data);
            match data[0] & 0x01 {
                0x00 => {
                    return Some(NusServiceEvent::TxCccdWrite { notifications: false });
                }
                0x01 => {
                    return Some(NusServiceEvent::TxCccdWrite { notifications: true });
                }
                _ => {}
            }
        }
        None
    }
}

pub enum NusServiceEvent {
    RxWrite(NusData),
    TxCccdWrite { notifications: bool },
}

#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) -> ! {
    sd.run().await
}

struct Server {
    nus: NusService,
}

impl Server {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let nus: NusService = NusService::new(sd)?;
        Ok(Self { nus: nus })
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
        self.nus.on_write(handle, data);
        None
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("NUS_SVC_UUID {}", NUS_SVC_UUID);
    const ATT_MTU: u16 = 256;
    const ATTR_TAB_SIZE: u32 = raw::BLE_GATTS_ATTR_TAB_SIZE_DEFAULT;
    // const ATTR_TAB_SIZE: u32 = raw::BLE_GATTS_ATTR_TAB_SIZE_MIN;

    let config = nrf_softdevice::Config {
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_XTAL as u8,
            rc_ctiv: 0,
            rc_temp_ctiv: 0,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_20_PPM as u8,
        }),
        conn_gap: Some(raw::ble_gap_conn_cfg_t {
            conn_count: 6,
            event_length: 24,
        }),
        conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: ATT_MTU }),
        // gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t { attr_tab_size: 32768 }),
        gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t {
            attr_tab_size: ATTR_TAB_SIZE,
        }),
        gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
            adv_set_count: 1,
            periph_role_count: 3,
            central_role_count: 3,
            central_sec_count: 0,
            _bitfield_1: raw::ble_gap_cfg_role_count_t::new_bitfield_1(0),
        }),
        gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
            p_value: b"Hello Rust Y" as *const u8 as _,
            current_len: 12,
            max_len: 12,
            write_perm: unsafe { mem::zeroed() },
            _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(raw::BLE_GATTS_VLOC_STACK as u8),
        }),
        ..Default::default()
    };

    let sd = Softdevice::enable(&config);
    let server = unwrap!(Server::new(sd));
    unwrap!(spawner.spawn(softdevice_task(sd)));

    // build advertisement data

    const NAME: &[u8; 6] = b"RV Log";

    // https://jimmywongiot.com/2019/08/13/advertising-payload-format-on-ble/
    #[rustfmt::skip]
    let adv_data: &[u8; 3+18+2+NAME.len()] = &concat_arrays!(
        [ 2u8, raw::BLE_GAP_AD_TYPE_FLAGS as u8, raw::BLE_GAP_ADV_FLAGS_LE_ONLY_GENERAL_DISC_MODE as u8, ],
        [ (NUS_SVC_UUID.len()+1) as u8, raw::BLE_GAP_AD_TYPE_128BIT_SERVICE_UUID_COMPLETE as u8 ], *NUS_SVC_UUID,
        [ (NAME.len() + 1) as u8, raw::BLE_GAP_AD_TYPE_COMPLETE_LOCAL_NAME as u8 ], *NAME
    );

    #[rustfmt::skip]
    let scan_data = &[];

    loop {
        let config = peripheral::Config::default();
        let adv = peripheral::ConnectableAdvertisement::ScannableUndirected { adv_data, scan_data };
        info!("advertise ...");
        let conn = unwrap!(peripheral::advertise_connectable(sd, adv, &config).await);

        info!("advertising done, in connection ...");

        let notify_fut = tx_notifier(&sd, &server, &conn);

        // Run the GATT server on the connection. This returns when the connection gets disconnected.
        let gatt_fut = gatt_server::run(&conn, &server, |e| info!("gatt_server::run {}", e));

        pin_mut!(notify_fut);
        pin_mut!(gatt_fut);

        let _ = match select(notify_fut, gatt_fut).await {
            Either::Left((_, _)) => {
                info!("tx_notify stopped")
            }
            Either::Right((res, _)) => {
                if let Err(e) = res {
                    info!("nus exited with error: {}", e)
                }
            }
        };

        info!("gatt_server::run returns (disconneced?)");
    }
}

async fn tx_notifier<'a>(sd: &'a Softdevice, server: &'a Server, connection: &'a Connection) {
    let mut n = 0;
    loop {
        let mut val = NusData::new();
        val.push(b'a' + (n % 26) as u8).unwrap();
        info!("tx_notify task: {}", val.as_slice());
        match server.nus.tx_notify(connection, &val) {
            Ok(_) => info!("tx {}", n),
            Err(e) => {
                info!("tx notify failed with error: {}", e);
                server.nus.tx_set(sd, &val);
            }
        }
        info!("sleep a little");
        Timer::after(Duration::from_secs(1)).await;
        info!("sleep done");
        n += 1;
    }
}
