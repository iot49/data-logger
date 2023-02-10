use defmt::*;
use super::scanner::ScanResult;

/// Govee Advertisement formats:
/// 
/// H5074
///  -48dBm T = 21.58C  H = 39.29%  batt = 100% uuid=[] addr=[fe, a6, 18, 38, c1, a4] uuid=[]
///  -50dBm T = 21.59C  H = 39.32%  batt = 100% uuid=[] addr=[fe, a6, 18, 38, c1, a4] uuid=[]
///  -43dBm T = 21.61C  H = 39.27%  batt = 100% uuid=[] addr=[fe, a6, 18, 38, c1, a4] uuid=[]
///  -47dBm T = 21.58C  H = 39.26%  batt = 100% uuid=[] addr=[fe, a6, 18, 38, c1, a4] uuid=[]
/// 
/// also with name but no data (could use for identification)
///  Govee_H5074_A6FE addr=[fe, a6, 18, 38, c1, a4] uuid=[88, ec] data=[]

/// 
/// H5075 - not quite right ...
///  -91dBm T = -2.54C  H = 0.59%  batt = 0% uuid=[88, ec] addr=[a2, 6d, 8b, 38, c1, a4] uuid=[88, ec] name=GVH5075_6DA2
///  GVH5075_6DA2     addr=[a2, 6d, 8b, 38, c1, a4] uuid=[88, ec] data=[88, ec, 0, 2, ff, 3c, 0, 0]
/// 


pub fn parse_adv(adv: &ScanResult) {
    let name = adv.name();
    let data = adv.manufacturer_data();
    let uuid = adv.uuid_16();
    if data.len() == 9 && data[0] == 0x88 && data[1] == 0xec {
        // very likely a Govee H5074, should be sufficient to identify
        let temp = i16::from_le_bytes(data[3..5].try_into().unwrap()) as f32 / 100.0;
        let humi = u16::from_le_bytes(data[5..7].try_into().unwrap()) as f32 / 100.0;
        let batt  = data[7];
        info!("{}dBm T = {}C  H = {}%  batt = {}% uuid={:x} addr={:x} uuid={:x} name={}", adv.rssi, temp, humi, batt, uuid, adv.mac_address, uuid, name);    
    } else if uuid.len() > 1 && uuid[0] == 0x88 {
        // other possible Govee's, captures H5075
        debug!("{} addr={:x} uuid={:x} data={:x}", name, adv.mac_address, uuid, data);
    }
}
