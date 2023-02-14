#![allow(dead_code)]

use defmt::*;
use embassy_sync::pubsub::PubSubChannel;
use embassy_sync::pubsub::publisher::{Publisher, ImmediatePublisher};
use embassy_sync::pubsub::subscriber::Subscriber;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use heapless::String as HeaplessString;
use static_cell::StaticCell;

use logger_lib::state_types::{Entity, Device, Attribute, Value};


pub type LogString = HeaplessString<64>;

pub struct Comm {
    /// state updates from sensors to StateFilter
    pub state_bus: Channel<CriticalSectionRawMutex, (Entity, f32), 8>,
    /// log messages to TX
    pub log_bus: Channel<CriticalSectionRawMutex, LogString, 2>,
}

impl Comm {

    pub const fn new() -> Self {
        Self {
            state_bus: Channel::new(),
            log_bus: Channel::new(),    
        }
    }

    pub fn publish_state(&self, device: Device, attr: Attribute, value: Value) {
        let entity = Entity { device: device, attr: attr };
        if self.state_bus.try_send((entity, value)).is_err() {
            self.log_str("Dropped state update (buffer full)");
        }
    }

    pub async fn log_str(&self, msg: &str) {
        let s = LogString::from(msg);
        self.log_bus.send(s).await;
    }

    pub async fn log_string(&self, msg: LogString) {
        self.log_bus.send(msg).await;
    }

}