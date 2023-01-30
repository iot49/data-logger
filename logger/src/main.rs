#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(default_alloc_error_handler)]

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

mod boards;
mod comm;
mod imu;
mod gps;
mod ble;
mod event_logger;

#[cfg(feature = "particle-xenon")]
use crate::boards::particle_xenon as bsp;
#[cfg(feature = "microbit-v2")]
use crate::boards::microbit_v2 as bsp;

const _HEAP_SIZE: usize = 128*1024;


#[embassy_executor::main]
async fn main(spawner: Spawner) {

    // disable to check stack usage (enable --measure-stack in config.toml)
    #[cfg(feature = "use-heap")]
    init_heap();

    // peripherals
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);
    let (gps_peripherals, imu_peripherals) = bsp::init(p);

    // Inter-Task Communication
    static STATE_BUS: comm::StateBus = comm::StateBus::new();
    
    // Logging
    unwrap!(spawner.spawn(event_logger::main_task(&STATE_BUS)));

    // IMU
    unwrap!(spawner.spawn(imu::main_task(imu_peripherals)));

    // GPS
    unwrap!(spawner.spawn(gps::main_task(gps_peripherals)));

    // Bluetooth
    let sd = ble::init::start_softdevice(spawner);
    unwrap!(spawner.spawn(ble::scanner::main_task(sd, &STATE_BUS)));

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