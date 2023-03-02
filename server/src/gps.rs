use crate::bsp;

use defmt::*;
use embassy_nrf::uarte::{self, Uarte};

/// Read NMEA GPS
#[embassy_executor::task(pool_size = 1)]
pub async fn main_task(mut uart: bsp::GpsUart) {
    info!("gps.main_task");
    let mut buf = [0; 100];
    for _i in 1..100 {
        uart.read(&mut buf).await.unwrap();
        if let Ok(s) = core::str::from_utf8(&buf) {
            if s.contains("$GNRMC") || s.contains("$GNGGA") {
                info!("{}", s[..20]);
            }    
        }
    }
}
