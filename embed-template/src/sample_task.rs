use crate::bsp;

use defmt::*;
use embassy_nrf::uarte::{self, Uarte};

/// Read NMEA GPS
#[embassy_executor::task(pool_size = 1)]
pub async fn main_task(p: bsp::GpsPeripherals) {
    debug!("gps.main_task");
    let mut uart = init_peripherals(p);

    debug!("Gps initialised");
    let mut buf = [0; 100];
    for _i in 1..100 {
        uart.read(&mut buf).await.unwrap();
        if let Ok(s) = core::str::from_utf8(&buf) {
            if s.contains("$GNRMC") || s.contains("$GNGGA") {
                debug!("{}", s[..20]);
            }    
        }
    }
}

fn init_peripherals<'a>(p: bsp::GpsPeripherals) -> Uarte<'a, bsp::GpsUarte> {
    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD9600;
    Uarte::new(
        p.uarte,
        p.uarte_interrupt,
        p.uarte_rx_pin,
        p.uarte_tx_pin,
        config,
    )
}
