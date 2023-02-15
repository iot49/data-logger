#[cfg(feature = "defmt")]
use defmt::Format;
use std::prelude::v1::*;
use serde::{Serialize, Deserialize};
use core::ops;

#[cfg(feature = "std")]
use std::time::Instant;
#[cfg(not(feature = "std"))]
use embassy_time::Instant;

/// Todo: [us] elapsed since 1970 (or whatever)
/// set with GPS time
const TIME_OFFSET: u64 = 927345092;


#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn now() -> Self {
        let now = Instant::now().elapsed().as_micros() as u64;
        Self(now + TIME_OFFSET)
    }

    pub fn as_sec(self) -> u32 {
        (self.0 / 1000000u64) as u32
    }

    pub fn as_micros(self) -> u64 {
        self.0
    }

    pub fn from_bytes(b: &[u8; 4]) -> Self {
        let e = u32::from_be_bytes(*b) as u64;
        Self(1000000 * e)
    }

    /// round to seconds (4 bytes only)
    pub fn to_bytes(self) -> [u8; 4] {
        self.as_sec().to_be_bytes()       
    }
    
}

impl ops::Sub<Timestamp> for Timestamp {
    type Output = f32;

    /// Time difference in seconds
    fn sub(self, _rhs: Timestamp) -> f32 {
        let delta = self.0 - _rhs.0;
        (delta as f32) / 1000000.0
    }
}


#[test]
fn attr_test() {
    let ts = Timestamp::now();
    assert_eq!(ts.as_sec(), (ts.0/1000000u64) as u32);
    let b = ts.to_bytes();
    let ts_ = Timestamp::from_bytes(&b);
    // the reconsistutes timestamp is rounded to seconds!
    assert_eq!(ts.as_sec(), ts_.as_sec());
}

