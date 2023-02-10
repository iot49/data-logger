use defmt::*;
use core::mem;
use nrf_softdevice::{raw, Softdevice, Config};

/// Broadcast name
pub const NAME: &[u8; 8] = b"Nus Test";

/// MUT, most iPhones support only 185 bytes
pub const ATT_MTU: u16 = 185;

const ATTR_TAB_SIZE: u32 = raw::BLE_GATTS_ATTR_TAB_SIZE_DEFAULT;


#[embassy_executor::task]
pub async fn softdevice_task(sd: &'static Softdevice) -> ! {
    debug!("start softdevice");
    sd.run().await
}


pub fn softdevice_config() -> nrf_softdevice::Config {
    nrf_softdevice::Config {
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
            p_value: NAME.as_ptr() as *const u8 as _,
            current_len: NAME.len() as u16,
            max_len: NAME.len() as u16,
            write_perm: unsafe { mem::zeroed() },
            _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(raw::BLE_GATTS_VLOC_STACK as u8),
        }),
        ..Default::default()
    }
    
}