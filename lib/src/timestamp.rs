#[cfg(feature = "defmt")]
use defmt::Format;
use embassy_time::Instant;
use core::ops;


/// Todo: [us] elapsed since 1970 (or whatever)
const TIME_OFFSET: u64 = 0;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct Timestamp {
    // micro-seconds since 1970
    epoch: u64
}

impl Timestamp {
    pub fn now() -> Self {
        Self {
            epoch: Instant::now().as_micros() + TIME_OFFSET
        }
    }

    pub fn as_sec(self) -> u32 {
        (self.epoch / 1000000u64) as u32
    }

    pub fn from_bytes(b: &[u8; 4]) -> Self {
        let e = u32::from_be_bytes(*b) as u64;
        Self {
            epoch: 1000000 * e
        }
    }

    /// round to seconds (4 bytes only)
    pub fn to_bytes(self) -> [u8; 4] {
        self.as_sec().to_be_bytes()       
    }
    
    pub fn epoch(self) -> u64 {
        return self.epoch
    }
    
}


impl ops::Sub<Timestamp> for Timestamp {
    type Output = f32;

    /// Time difference in seconds
    fn sub(self, _rhs: Timestamp) -> f32 {
        let delta = self.epoch - _rhs.epoch;
        (delta as f32) / 1000000.0
    }
}