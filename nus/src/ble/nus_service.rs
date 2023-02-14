use defmt::*;
use uuid::uuid;
use heapless::Vec as HeaplessVec;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{NotifyValueError, RegisterError, Service, WriteOp, indicate_value};
use nrf_softdevice::ble::{gatt_server, peripheral, Connection, GattValue, Uuid};
use nrf_softdevice::{raw, Softdevice};
use embassy_time::{Timer, Duration};

use crate::comm::Comm;

pub const NUS_SV_UUID: &[u8; 16] = &uuid!("6e400001-b5a3-f393-e0a9-e50e24dcca9e").to_u128_le().to_be_bytes();
pub const NUS_RX_UUID: &[u8; 16] = &uuid!("6e400002-b5a3-f393-e0a9-e50e24dcca9e").to_u128_le().to_be_bytes();
pub const NUS_TX_UUID: &[u8; 16] = &uuid!("6e400003-b5a3-f393-e0a9-e50e24dcca9e").to_u128_le().to_be_bytes();

// const NUS_MAX_LEN: usize = 20;
const NUS_MAX_LEN: usize = 180;

pub type NusData = HeaplessVec<u8, NUS_MAX_LEN>;

/// Service Handles
pub struct NusService {
    rx_valh: u16,
    tx_valh: u16,
    tx_cccd: u16,
}

impl NusService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let uuid = Uuid::new_128(NUS_SV_UUID);
        let mut svc = ServiceBuilder::new(sd, uuid)?;

        let attr = Attribute::new(&[0u8])
            // .write_security(SecurityMode::JustWorks)
            // .deferred_write()
            .variable_len(NUS_MAX_LEN as u16)
            ;
        let props = Properties::new()
            .write()
            .write_without_response()
            ;
        let uuid = Uuid::new_128(NUS_RX_UUID);
        let builder = svc.add_characteristic(uuid, attr, Metadata::new(props))?;
        let c_rx = builder.build();

        let attr = Attribute::new(&[0u8]).variable_len(NUS_MAX_LEN as u16);
        let props = Properties::new()
            // .read()
            .notify()
            ;
        let uuid = Uuid::new_128(NUS_TX_UUID);
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
        // info!("tx_notify {}", buf);
        gatt_server::notify_value(conn, self.tx_valh, buf)
    }

}


impl Service for NusService {
    type Event = NusServiceEvent;

    /// receiver
    fn on_write(&self, handle: u16, data: &[u8]) -> Option<Self::Event> {
        // data received from central
        if let Ok(s) = core::str::from_utf8(&data) {
            info!("on_write as str '{}'", s);
        } else {
            info!("on_write as [u8] '{}'", data);
        }
        if handle == self.rx_valh {
            debug!("rx_valh {}", self.rx_valh);
            return Some(NusServiceEvent::RxWrite(
                <NusData as ::nrf_softdevice::ble::GattValue>::from_gatt(data),
            ));
        }
        if handle == self.tx_cccd && !data.is_empty() {
            debug!("nus notifications: {}", (data[0] & 0x01) != 0);
            return Some(NusServiceEvent::TxCccdWrite { notifications: (data[0] & 0x01) != 0 });
        }
        None
    }
}


pub enum NusServiceEvent {
    RxWrite(NusData),
    TxCccdWrite { notifications: bool },
}

/// transmitter
pub async fn nus_fut<'a>(comm: &'a Comm, service: &'a NusService, conn: &'a Connection) {
    debug!("nus_fut started");
    loop {
        debug!("nus_fut await ...");
        let s = comm.log_bus.recv().await;
        let val = NusData::from_slice(s.as_bytes()).unwrap();
        debug!("nus_fut tx {}", s.as_str());
        let _ = service.tx_notify(conn, &val);
    }
}
