use defmt::*;
use nrf_softdevice::ble::central;
use nrf_softdevice::{raw, Softdevice};
use nrf_softdevice_s140::ble_gap_scan_params_t;
use core::slice;
use heapless::FnvIndexMap as HeaplessMap;
use super::govee;

pub type BleMacAddress = [u8; 6];
pub type ScanData<'a> = HeaplessMap<u32, &'a [u8], 8>;

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
        if let Some(v) = self.val(raw::BLE_GAP_AD_TYPE_FLAGS) {
            v[0]
        } else {
            0
        }
    }

    /// Complete or short name, empty string if not available
    pub fn name(&self) -> &str {
        if let Some(v) = self.val(raw::BLE_GAP_AD_TYPE_COMPLETE_LOCAL_NAME) {
            if let Ok(s) = core::str::from_utf8(v) {
                return s
            } else if let Some(v) = self.val(raw::BLE_GAP_AD_TYPE_SHORT_LOCAL_NAME) {
                if let Ok(s) = core::str::from_utf8(v) {
                    return s
                }
            }
        }
        ""
    }

    /// Manufacturer data (key 0xff), empty array if none
    pub fn manufacturer_data(&self) -> &[u8] {
        if let Some(v) = self.val(raw::BLE_GAP_AD_TYPE_MANUFACTURER_SPECIFIC_DATA) {
            v
        } else {
            &[]
        }
    }

    /// Complete or incomplete list of advertised 16-bit UUIDs, empty array if none
    pub fn uuid_16(&self) -> &[u8] {
        if let Some(v) = self.val(raw::BLE_GAP_AD_TYPE_16BIT_SERVICE_UUID_COMPLETE) {
            return v
        } else if let Some(v) = self.val(raw::BLE_GAP_AD_TYPE_16BIT_SERVICE_UUID_MORE_AVAILABLE) {
            return v
        }
        &[]
    }

}


/// Continuously scan for BLE advertisements
#[embassy_executor::task]
pub async fn main_task(sd: &'static Softdevice) {
    debug!("scanner::main_task started");
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
        // notify all interested parties ...
        govee::parse_adv(&sr);
        debug!("Scan result for {} with uuids {:x}: {:x} {}dBm", sr.name(), sr.uuid_16(), sr.mac_address, sr.rssi);
        None
    }).await;
    unwrap!(res);
    error!("scanner::main_task returned");
}