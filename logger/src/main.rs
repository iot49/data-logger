#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use defmt::{debug, unwrap};
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_nrf::interrupt::Priority;

use logger_lib::comm::StateBus;

mod boards;
mod gps;
mod ble;
mod event_logger;

#[cfg(feature = "particle-xenon")]
use crate::boards::particle_xenon as bsp;
#[cfg(feature = "microbit-v2")]
use crate::boards::microbit_v2 as bsp;



#[embassy_executor::main]
async fn main(spawner: Spawner) {
    debug!("Data-Logger starting");

    // peripherals
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);

    let gps_peripherals = bsp::init(p);

    // Inter-Task Communication
    static STATE_BUS: StateBus = StateBus::new();
    
    // Logging
    unwrap!(spawner.spawn(event_logger::main_task(&STATE_BUS)));

    // GPS
     unwrap!(spawner.spawn(gps::main_task(gps_peripherals)));

    // Bluetooth
    let sd = ble::init::start_softdevice(spawner);
    unwrap!(spawner.spawn(ble::scanner::main_task(sd, &STATE_BUS)));

}
