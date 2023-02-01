#[cfg(feature = "defmt")]
use defmt::Format;
use embassy_time::Instant;


#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct State {
    pub timestamp: Timestamp,
    pub entity: Entity,
    pub value: f32,
}

impl State {
    pub fn new(device: Device, attr: Attribute, value: f32) -> Self {
        Self {
            timestamp: Timestamp::now(),
            entity: Entity { device: device, attr: attr },
            value: value,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct Timestamp {
    epoch: usize
}

impl Timestamp {
    pub fn now() -> Self {
        Self {
            // TODO: set correct epoch, e.g. sec since 2000
            epoch: Instant::now().as_secs() as usize
        }
    }
    pub fn epoch(&self) -> usize {
        return self.epoch
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
    #[allow(dead_code)]
    fn new(kind: DeviceKind, instance: u8, attr: Attribute) -> Self {
        Self { 
            device: Device {
                kind: kind,
                instance: instance
            }, 
            attr: attr
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct Device {
    pub kind: DeviceKind,
    pub instance: u8,
}

impl Device {
    pub fn new(kind: DeviceKind, instance: u8) -> Self {
        assert!(instance < 255);
        Self {
            kind: kind,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[repr(u8)]
pub enum DeviceKind {
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

/// Attributes of devices. 
/// Limitation: each device may have only one instance of
/// a particular attribute, e.g. Current. More general situations,
/// e.g. input current and output current require two separate devices.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
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

pub const UNKNOWN: f32 = f32::NAN;
pub const ON: f32 = 1.0;
pub const OFF: f32 = 0.0;
