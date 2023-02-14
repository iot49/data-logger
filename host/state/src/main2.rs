
use logger_lib::state_types::*;


fn _state_test() {
    let dev = Device::new(DeviceKind::Climate, 0);
    let s1 = State::new(dev, Attribute::Temperature, 21.4);
    let mut buf = [0u8; 11];
    s1.to_bytes(&mut buf);
    let s2 = State::from_bytes(&buf);
    println!("{:#?}", buf);
    println!("{:#?}\n{:#?}", s1, s2);
    assert_eq!(s1, s2);
    assert_eq!(s1.entity, s2.entity);
    assert_eq!(s1.timestamp, s2.timestamp);
    assert_eq!(s1.value, s2.value);
}

use heapless::Vec as HVec;

fn y(buf: &mut HVec) {
    buf.push(5u8);
}

fn x(buf: &mut [u8]) {
    buf[2] = 22;
    buf[4..8].copy_from_slice(&4.3f32.to_be_bytes());
    println!("len = {} {:?}", buf.len(), buf);
}

fn main() {
    let mut b = [0u8; 11];
    b[3] = 33;
    x(&mut b);
    let mut v = HVec::<u8, 20>::new();
    let s = [0u8, 1, 2];
    v.extend_from_slice(&s);
    println!("v = {:#?}", v);

    let mut hv = HVec::<u8,16>::new();
    y(hv);
}