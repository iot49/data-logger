#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![allow(unused)]

// Global Heap, see https://github.com/peterstuart/cherry/tree/main/examples/ssd1306
// https://github.com/rust-embedded/embedded-alloc/blob/master/examples/global_alloc.rs

#[cfg(feature = "use-heap")]
extern crate alloc;
#[cfg(feature = "use-heap")]
use alloc_cortex_m::CortexMHeap;

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_nrf::interrupt::Priority;
use nrf_softdevice::Softdevice;
use embassy_time::{Timer, Duration};
use futures::future::{select, Either};
use futures::pin_mut;

mod boards;
mod ble;

#[cfg(feature = "particle-xenon")]
use crate::boards::particle_xenon as bsp;
#[cfg(feature = "microbit-v2")]
use crate::boards::microbit_v2 as bsp;

const _HEAP_SIZE: usize = 128*1024;


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting ...");

    // optionally initialize heap (see Cargo.toml)
    // IMPORTANT! disable to check stack usage 
    //            (enable --measure-stack in config.toml)
    #[cfg(feature = "use-heap")]
    init_heap();

    // remap interrupts for soft-device
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);

    // get peripherals
    let (gps_peripherals, imu_peripherals) = bsp::init(p);

    info!("initializations done, now start the app ...");

    // Bluetooth
    //let sd = ble::config::start_softdevice(spawner, "Nus Test");
    unwrap!(spawner.spawn(ble::x::main_task()));

    info!("main: tasks started.");

}



#[cfg(feature = "use-heap")]
fn init_heap() {
    static mut HEAP: [u8; _HEAP_SIZE] = [0; _HEAP_SIZE];
    
    #[global_allocator]
    static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
    unsafe {
        ALLOCATOR.init(
            &mut HEAP as *const u8 as usize,
            core::mem::size_of_val(&HEAP),
        )
    }
}

