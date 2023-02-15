use logger_lib::state_types::*;
use embedded_crc_macros::crc8;

crc8!(fn crc8, 7, 0, "State Packet CRC Error");


fn main() {
    let dev = Device::new(DeviceType::Climate, 0);
    let s1 = State::new(dev, Attribute::Temperature, 21.4);
    let s2 = State::new(dev, Attribute::Humidity, 33.0);

    const N: usize = 30;
    let mut buf = heapless::Vec::<u8, N>::new();

    s1.to_bytes::<N>(&mut buf).unwrap();
    let size = buf.len();
    s2.to_bytes::<N>(&mut buf).unwrap();
    let _ = buf.push(4);

    let mut iter = buf.chunks_exact(size);
    let s1_ = State::from_bytes(iter.next().unwrap()).unwrap();
    let s2_ = State::from_bytes(iter.next().unwrap()).unwrap();

    assert_eq!(s1, s1_);
    assert_eq!(s2, s2_);

}