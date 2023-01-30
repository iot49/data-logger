#[cfg(feature = "defmt")]
use defmt::Format;
use embassy_time::Instant;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub enum Event {
    StateUpdate(State),
    StateReport(State),
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct State {
    pub timestamp: Timestamp,
    pub entity: Entity,
    pub value: Value,
}

impl State {
    pub fn new(dev: DeviceInstance, attr: Attribute, value: f32) -> Self {
        Self {
            timestamp: Timestamp::now(),
            entity: Entity { device_instance: dev, attr: attr },
            value: Value::Number(value)
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
/// Example:
/// ```
/// let entity = Entity {
///     device: Device::Gps,
///     instance: 0,
///     attr: Attribute::Longitude
/// };
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct Entity {
    pub device_instance: DeviceInstance,
    pub attr: Attribute
}

impl Default for Entity {
    fn default() -> Self {
        Self { 
            device_instance: DeviceInstance::default(), 
            attr: Attribute::Unknown 
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct DeviceInstance {
    pub device: Device,
    pub instance: u8,
}

impl Default for DeviceInstance {
    fn default() -> Self {
        Self { 
            device: Device::Forbidden, 
            instance: 0, 
        }
    }
}

impl DeviceInstance {
    pub fn new(device: Device, instance: u8) -> Self {
        assert!(instance < 255);
        Self {
            device: device,
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
pub enum Device {
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
/// 
/// TODO: consider combining with Value, e.g. Temperature(f32), Binary(OnOff)
///       or support only numeric values? Unknown = NaN?
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
}

use core::fmt;
impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Attribute::Unknown => write!(f, "Unknown"),
           Attribute::Temperature => write!(f, "T"),
           _ => write!(f, "?")
       }
    }
}

/// Value of an Entity.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub enum Value {
    Unknown,
    Number(f32),
    On,
    Off,
}

impl Default for Value {
    fn default() -> Self {
        Value::Unknown
    }
}