use defmt::*;
use nrf_softdevice::ble::central;
use nrf_softdevice::{raw, Softdevice};
use nrf_softdevice_s140::ble_gap_scan_params_t;
use core::slice;
use heapless::LinearMap as HeaplessMap;

use crate::comm::Comm;
use logger_lib::state_types::*;

pub type BleMacAddress = [u8; 6];
pub type ScanData<'a> = HeaplessMap<u32, &'a [u8], 6>;

pub struct ScanResult<'a> {
    pub mac_address: BleMacAddress,
    pub rssi: i8,
    pub data: ScanData<'a>,
}

impl ScanResult<'_> {

    /// advertisement for specified key
    pub fn val(&self, key: u32) -> Option<&[u8]> {
       if let Some(v) = self.data.get(&key) {
            return Some(*v)
        }
        None
    }

    /// advertisement flags, 0 if not in advertising message
    pub fn type_flags(&self) -> u8 {
        self.val(raw::BLE_GAP_AD_TYPE_FLAGS).unwrap_or(&[0])[0]
    }

    /// Complete or short name, empty string if not available
    pub fn name(&self) -> &str {
        if let Some(v) = self.val(raw::BLE_GAP_AD_TYPE_COMPLETE_LOCAL_NAME) {
            core::str::from_utf8(v).unwrap_or("")
        } else if let Some(v) = self.val(raw::BLE_GAP_AD_TYPE_SHORT_LOCAL_NAME) {
            core::str::from_utf8(v).unwrap_or("")
        } else {
            ""
        }
    }

    /// Manufacturer data (key 0xff), empty array if none
    pub fn manufacturer_data(&self) -> &[u8] {
        self.val(raw::BLE_GAP_AD_TYPE_MANUFACTURER_SPECIFIC_DATA).unwrap_or(&[])
    }

    /// Complete or incomplete list of advertised 16-bit UUIDs, empty array if none
    pub fn uuid_16(&self) -> &[u8] {
        if let Some(v) = self.val(raw::BLE_GAP_AD_TYPE_16BIT_SERVICE_UUID_COMPLETE) {
            v
        } else {
            self.val(raw::BLE_GAP_AD_TYPE_16BIT_SERVICE_UUID_MORE_AVAILABLE).unwrap_or(&[])
        }
    }

}


/// Continuously scan for BLE advertisements
#[embassy_executor::task]
pub async fn main_task(sd: &'static Softdevice, comm: &'static Comm) {
    debug!("scanner::main_task starting");
    let mut config = central::ScanConfig::default();
    let res = central::scan(sd, &config, |params| unsafe {
        let mut data = slice::from_raw_parts(params.data.p_data, params.data.len as usize);
        let mut sr = ScanResult {
            mac_address: params.peer_addr.addr,
            rssi: params.rssi,
            data: ScanData::new()
        };
        while data.len() != 0 {
            let len = data[0] as usize;
            // ignore ill-formed advertisement data
            if len < 1 || data.len() < len + 1 { break; }
            let key = data[1];
            let value = &data[2..len + 1];
            let _ = sr.data.insert(key as u32, value);
            data = &data[len + 1..];
        };
        parse_adv(&sr, comm);
        debug!("Scan result for {} with uuids {:x}: {:x} {}dBm", sr.name(), sr.uuid_16(), sr.mac_address, sr.rssi);
        None
    }).await;
    unwrap!(res);
    error!("scanner::main_task returned");
}


pub fn parse_adv(adv: &ScanResult, comm: &Comm) {
    let data = adv.manufacturer_data();
    if data.len() == 9 && data[0] == 0x88 && data[1] == 0xec {
        // very likely a Govee H5074, should be sufficient to identify
        let temp = i16::from_le_bytes(data[3..5].try_into().unwrap()) as Value / 100.0;
        let humi = u16::from_le_bytes(data[5..7].try_into().unwrap()) as Value / 100.0;
        let batt  = data[7];
        let dev = Device::new(DeviceType::Climate, 3);
        comm.publish_state(dev, Attribute::Temperature, temp);
        comm.publish_state(dev, Attribute::Humidity, humi);
        comm.publish_state(dev, Attribute::BatteryLevel, batt as Value);
        comm.publish_state(dev, Attribute::Rssi, adv.rssi as Value);
        debug!("{:x} {}dBm T = {}C  H = {}%  batt = {}%", adv.mac_address, adv.rssi, temp, humi, batt);
    }
    // add other devices (e.g. Victron) here ...
}
