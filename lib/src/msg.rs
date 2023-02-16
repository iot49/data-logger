/// RX/TX messages sent between server and client
/// 

use std::prelude::v1::*;
use serde::{Serialize, Deserialize};

use super::state_types::{Device, Entity, State};

#[derive(Serialize, Deserialize, Debug)]
enum Cmd {
    /// Update state value
    UpdateState(State),
    /// Send history for specified entity
    GetHistory(Entity),
    History{entity: Entity, values: Vec::<State>},
    /// Send all current state values
    SendState,

    /// Log message
    Log(String),

    /// Devices on/off
    OnOff{ dev: Device, brightness: f32 },

    /// Get/send files
    GetFile(String),
    FileData(String, Vec::<u8>),
    ListDir,
}


