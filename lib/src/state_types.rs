#[cfg(feature = "defmt")]
use defmt::Format;
use super::timestamp::Timestamp;
use embedded_crc_macros::crc8;

crc8!(fn crc8, 7, 0, "State Packet CRC Error");

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct State {
    pub timestamp: Timestamp,
    pub entity: Entity,
    pub value: Value,
}

impl State {
    pub fn new(device: Device, attr: Attribute, value: Value) -> Self {
        Self {
            timestamp: Timestamp::now(),
            entity: Entity { device: device, attr: attr },
            value: value,
        }
    }

    /// Append binary representation of self to buf.
    /// The size (of the present implementation) is 12 bytes.
    /// The last byte is the crc, checked when reading back with `from_bytes`.
    pub fn to_bytes<const N: usize>(&self, buf: &mut heapless::Vec<u8, N>) -> Result<(), u8> {
        let offset = buf.len();
        // Note: catch error only on last push
        let _ = buf.push(self.entity.device.device_type.to_u8());
        let _ = buf.push(self.entity.device.instance);
        let _ = buf.push(self.entity.attr.to_u8());
        let _ = buf.extend_from_slice(&self.value.to_be_bytes());
        let _ = buf.extend_from_slice(&self.timestamp.to_bytes());
        
        // compute crc
        let crc = crc8(&buf[offset..]);
        buf.push(crc)
    }

    /// Reconstruct from byte array.
    pub fn from_bytes(src: &[u8]) -> Result<Self, ()> {
        // check CRC
        let crc = crc8(&src[..11]);
        if crc != src[11] { return Err(()); }
        Ok(Self {
            timestamp: Timestamp::from_bytes(src[7..11].try_into().unwrap()),
            entity: Entity { 
                device: Device { 
                    device_type: DeviceType::from_u8(src[0]), 
                    instance: src[1],
                },
                attr: Attribute::from_u8(src[2])
            },
            value: Value::from_be_bytes(src[3..7].try_into().unwrap())
        })
    }
}

/// Identifies a state, e.g. the temperature of a climate sensor.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct Entity {
    pub device: Device,
    pub attr: Attribute
}

impl Entity {
    pub fn new(device_type: DeviceType, instance: u8, attr: Attribute) -> Self {
        Self { 
            device: Device {
                device_type: device_type,
                instance: instance
            }, 
            attr: attr
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct Device {
    pub device_type: DeviceType,
    pub instance: u8,
}

impl Device {
    pub fn new(device_type: DeviceType, instance: u8) -> Self {
        assert!(instance < 255);
        Self {
            device_type: device_type,
            instance: instance
        }
    }
}


/// Identifies a (physical) device, e.g. a `Gps`.
/// Devices have attributes, e.g. `Longitude` and `Lattide` for a GPS,
/// or Temperature and Humidity for a Climate sensor.
/// Multiple instances may exist of some devices,
/// e.g. `Tank 1`, `Tank 2`, etc.
/// Device is also used to identify unprogrammed flash:
/// the "Forbidden" state is used for this purpose and
/// may not be used for an actual device.
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[repr(u8)]
pub enum DeviceType {
    Unknown,
    Light,
    Switch,
    Tank,
    Climate,
    Gps,
    Button,
    Led,
    /// Markes unprogrammed cell in NOR flash
    Forbidden = 0xff
}

impl DeviceType {
    pub fn from_u8(x: u8) -> Self {
        num_traits::FromPrimitive::from_u8(x).unwrap_or(DeviceType::Unknown)
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

/// Attributes of devices. 
/// Limitation: each device may have only one instance of
/// a particular attribute, e.g. Current. More general situations,
/// e.g. input current and output current require two separate devices.
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[repr(u8)]
pub enum Attribute {
    Unknown,
    Voltage,      // [V]
    Current,      // [A]
    Power,        // [W]
    Energy,       // [Wh] - no Joules!
    Temperature,  // [C]
    Humidity,     // [%]
    Rssi,         // [dBm]
    BatteryLevel, // [%]
    TankLevel,    // [%]
    Brightness,   // [%]
    Binary,       // [On/Off]
    Longitude,    // [deg]
    Latitude,     // [deg]
    Forbidden = 0xff
}

impl Attribute {
    pub fn from_u8(x: u8) -> Self {
        num_traits::FromPrimitive::from_u8(x).unwrap_or(Attribute::Unknown)
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

pub type Value = f32;

pub const UNKNOWN: Value = Value::NAN;
pub const ON: Value = 1.0;
pub const OFF: Value = 0.0;


#[test]
fn attr_test() {
    let a = Attribute::Brightness;
    assert_eq!(a, Attribute::from_u8(a.to_u8()));
}

#[test]
fn state_test() {
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
    assert_eq!(State::from_bytes(iter.next().unwrap()).unwrap(), s1);
    assert_eq!(State::from_bytes(iter.next().unwrap()).unwrap(), s2);

}