/// RX/TX messages sent between server and client
/// 

#[cfg(feature = "defmt")]
use defmt::Format;
use std::prelude::v1::*;
use serde::{Serialize, Deserialize};
use super::state_types::{Device, Entity, State};

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "defmt", derive(Format))]
enum Cmd {
    /// Update state value
    Update(State),
    /// Send history for specified entity
    GetHistory(Entity),
    SendHistory{entity: Entity, values: Vec::<State>},
    /// Send all current state values
    Current,

    /// Log message
    Log(String),

    /// Devices on/off
    OnOff{ dev: Device, brightness: f32 },

    /// Get/send files
    PutFile(String),
    GetFile(String),
    FileData(Vec::<u8>),
    CloseFile,
    Dir,
}
