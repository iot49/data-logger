use crate::bsp;

use defmt::{info, debug};
use embassy_nrf::uarte::{self, Uarte};

/// Read NMEA GPS
#[embassy_executor::task(pool_size = 1)]
pub async fn main_task(p: bsp::GpsPeripherals) {
    debug!("gps.main_task");
    let mut uart = init_peripherals(p);

    debug!("Gps initialised");
    let mut buf = [0; 1024];
    for _i in 1..1000 {
        uart.read(&mut buf).await.unwrap();
        let s = core::str::from_utf8(&buf).unwrap();
        info!("gps [{}] {}", s.len(), s);
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
