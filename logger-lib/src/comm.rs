use embassy_sync::pubsub::PubSubChannel;
use embassy_sync::pubsub::publisher::{Publisher, ImmediatePublisher};
use embassy_sync::pubsub::subscriber::Subscriber;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

use crate::state_types::State;

const CAP : usize = 10;
const SUBS: usize =  2;
const PUBS: usize =  4;

pub type StateBus = PubSubChannel::<CriticalSectionRawMutex, State, CAP, SUBS, PUBS>;
pub type StatePub<'a> = Publisher <'a, CriticalSectionRawMutex, State, CAP, SUBS, PUBS>;
pub type ImmediateStatePub<'a> = ImmediatePublisher <'a, CriticalSectionRawMutex, State, CAP, SUBS, PUBS>;
pub type StateSub<'a> = Subscriber<'a, CriticalSectionRawMutex, State, CAP, SUBS, PUBS>;
