#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![allow(unused)]

extern crate no_std_compat as std;
use std::prelude::v1::*;

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
mod comm;
mod ble;
mod gps;
mod imu;
mod states;

#[cfg(feature = "particle-xenon")]
use crate::boards::particle_xenon as bsp;
#[cfg(feature = "microbit-v2")]
use crate::boards::microbit_v2 as bsp;

/// deliberately small during development to assess space requirements
const _HEAP_SIZE: usize = 16*1024;


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting ...");

    // optionally initialize heap (see Cargo.toml)
    // IMPORTANT! disable to check stack usage 
    //            (enable --measure-stack in config.toml)
    #[cfg(feature = "use-heap")]
    init_heap();

    // communication channels
    static COMM: comm::Comm = comm::Comm::new();

    logger_lib::msg::foo();

    // remap interrupts for soft-device
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);

    // get peripherals
    let (gps_peripherals, imu_peripherals) = bsp::init(p);

    // IMU
    unwrap!(spawner.spawn(imu::main_task(imu_peripherals)));

    // GPS
    unwrap!(spawner.spawn(gps::main_task(gps_peripherals)));

    // Bluetooth
    unwrap!(spawner.spawn(ble::main_task(&COMM)));

    // State filter
    unwrap!(spawner.spawn(states::main_task(&COMM)));

    info!("main: tasks started.");

    loop {
        for i in 0..255 {
            let c = b'a' + (i%26);
            let mut s = comm::LogString::from("Log ");
            s.push(c as char);
            COMM.log_string(s).await;
            Timer::after(Duration::from_secs(1)).await;
        }
    }

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

