use defmt::Format;
use usize;

#[derive(Copy, Clone, Format, Debug)]
pub enum Event {
    StateUpdate(State),
    StateReport(State),
}

#[derive(Copy, Clone, Format, Debug)]
pub struct State {
    pub timestamp: Timestamp,
    pub entity: Entity,
    pub value: Value,
}

#[derive(Copy, Clone, Format, Debug)]
pub struct Timestamp {
    epoch: usize
}

impl Timestamp {
    pub fn now() -> Self {
        Self {
            // TODO: set correct epoch, e.g. sec since 2000
            epoch: 0 
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
#[derive(Eq, PartialEq, Copy, Clone, Format, Debug)]
pub struct Entity {
    pub device: Device,
    pub instance: u8,
    pub attr: Attribute
}

impl Default for Entity {
    fn default() -> Self {
        Entity { 
            device: Device::Forbidden, 
            instance: 0, 
            attr: Attribute::Unknown 
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
#[derive(PartialEq, Eq, Copy, Clone, Format, Debug)]
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
#[derive(PartialEq, Eq, Copy, Clone, Format, Debug)]
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

/// Value of an Entity.
#[derive(Copy, Clone, Format, Debug)]
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