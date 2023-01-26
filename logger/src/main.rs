#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{info, unwrap};
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;

mod boards;
mod gps;

#[cfg(feature = "particle-xenon")]
use crate::boards::particle_xenon as bsp;
#[cfg(feature = "microbit-v2")]
use crate::boards::microbit_v2 as bsp;


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Logger starting");

    let p = embassy_nrf::init(embassy_nrf::config::Config::default());

    // The general idea is to initialise the board
    // specific peripherals that we will be using.
    // This often ends up being an assignment to
    // a tuple of peripherals.
    info!("call init bsp ...");
    let gps_peripherals = bsp::init(p);

    // We generally create a task per component
    // that ends up owning a number of peripherals.
    // There are a number of tasks like this and
    // we use either signals or channels to
    // communicate with them.
    unwrap!(spawner.spawn(gps::main_task(gps_peripherals,)));

    // We end up here normally with a loop and something
    // "main-like" that executes for your application,
    // often with the ability to communicate to the other
    // tasks via signals and channels etc.
}
