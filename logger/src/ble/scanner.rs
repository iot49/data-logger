// advertisement keys: https://infocenter.nordicsemi.com/index.jsp?topic=%2Fcom.nordic.infocenter.s140.api.v6.0.0%2Fgroup___b_l_e___g_a_p___a_d_v___t_y_p_e_s.html
// adv types: https://infocenter.nordicsemi.com/index.jsp?topic=%2Fcom.nordic.infocenter.s140.api.v6.0.0%2Fgroup___b_l_e___g_a_p___a_d_v___t_y_p_e_s.html
// "tutorial": https://community.silabs.com/s/article/kba-bt-0201-bluetooth-advertising-data-basics?language=en_US
// service ids: https://btprodspecificationrefs.blob.core.windows.net/assigned-values/16-bit%20UUID%20Numbers%20Document.pdf
// s140 api: https://infocenter.nordicsemi.com/index.jsp?topic=%2Fug_gsg_ses%2FUG%2Fgsg%2Fsoftdevices.html


use core::slice;
use defmt::*;
use nrf_softdevice::ble::central;
use nrf_softdevice::Softdevice;
use nrf_softdevice::ble::AddressType;
use nrf_softdevice::ble::Address;

use logger_lib::comm::{StateBus, StatePub};
use logger_lib::state_types::*;

const ADDRESS_FILTER: bool = true;

type BleMacAddress = [u8; 6];

fn scan_cb(_state_pub: &StatePub<'_>, addr: &BleMacAddress, key: u8, data: &[u8], rssi: i8) {
    match key {
        0x09 => { // name
            let name = core::str::from_utf8(&data).unwrap();
            if name.starts_with("Govee_H5074") {
                debug!("{:x} {}dBm {}", addr, rssi, name);
            }
        },
        0xff => { // manufacturer data
            if data.len() == 9 && data[0] == 0x88 && data[1] == 0xec {
                let temp = i16::from_le_bytes(data[3..5].try_into().unwrap()) as f32 / 100.0;
                let humi = u16::from_le_bytes(data[5..7].try_into().unwrap()) as f32 / 100.0;
                let batt  = data[7];
                let s = State {
                    timestamp: Timestamp::now(),
                    entity: Entity { 
                        device: Device::Climate, 
                        instance: 4, 
                        attr: Attribute::Temperature 
                    },
                    value: Value::Number(temp)
                };
                debug!("{}", s);
                match _state_pub.try_publish(s) {
                    Ok(()) => {},
                    Err(e) => error!("failed to send state {} to state_bus: {}", s, e)
                }            
                debug!("{:x} {}dBm T = {}C  H = {}%  batt = {}%", addr, rssi, temp, humi, batt);
            } else {
                debug!("{:x} {}dBm 0x{:02x} len={} {:x}", addr, rssi, key, data.len(), data);
            }
        },
        0x01 => {
        },
        _ => {
            debug!("{:x} {}dBm 0x{:02x} len={} {:x}", addr, rssi, key, data.len(), data);
        }
    }
}



#[embassy_executor::task]
pub async fn main_task(sd: &'static Softdevice, comm: &'static StateBus) {
    debug!("scanner::main_task started");
    let state_pub: StatePub = comm.publisher().unwrap();
    // filter advertisements by peer addresses
    let mut config = central::ScanConfig::default();
    let address = Address::new(AddressType::Public, [0x84, 0x8c, 0x26, 0x38, 0xc1, 0xa4]);
    let addresses = [&address];
    if ADDRESS_FILTER {
        config.whitelist = Option::Some(&addresses);
    }
    let res = central::scan(sd, &config, |params| unsafe {
        let mut data = slice::from_raw_parts(params.data.p_data, params.data.len as usize);
        while data.len() != 0 {
            let len = data[0] as usize;
            if data.len() < len + 1 {
                // warn!("Advertisement data truncated?");
                break;
            }
            if len < 1 {
                // warn!("Advertisement data malformed?");
                break;
            }
            let key = data[1];
            let value = &data[2..len + 1];
            scan_cb(&state_pub, &params.peer_addr.addr, key, value, params.rssi);  
            data = &data[len + 1..];
        }
        None
    }).await;
    unwrap!(res);
    error!("scanner::main_task returned");
}