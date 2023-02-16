// use serde::{Serialize, Deserialize};
use embedded_msgpack as emp;
use logger_lib::state_types::*;

// entity (DeviceType, ID, Attribute), value, timestamp
type S = (DeviceType, u8, Attribute, f32, u32);


pub fn main() {
    let mut buf = [0u8; 100];

    let a = Attribute::Battery_Level;
    let len = emp::encode::serde::to_array(&a, &mut buf).unwrap();
    let a_: Attribute = emp::decode::from_slice(&buf[..len]).unwrap();
    println!("{:#?} -> [{}] {:0x?} -> {:#?}", a, len, &buf[..len], a_);

    let dev = Device::new(DeviceType::Climate, 32);
    let s = State::new(dev, Attribute::Rssi, 3.14158234);
    let len = emp::encode::serde::to_array(&s, &mut buf).unwrap();
    let s_: State = emp::decode::from_slice(&buf[..len]).unwrap();
    assert_eq!(s, s_);
    println!("[{}] -> {:?}", len, &buf[..len]);

    let x: S = (DeviceType::Climate, 5u8, Attribute::Temperature, 3.492, 790);
    let len = emp::encode::serde::to_array(&x, &mut buf).unwrap();
    let x_: S = emp::decode::from_slice(&buf[..len]).unwrap();
    // assert_eq!(x, x_);
    println!("{:?} -> [{}] {:?} -> {:?}", x, len, &buf[..len], x_);
}