use crate::bsp;

use defmt::*;
use embassy_nrf::twim::{self, Twim};

use bno055::Bno055;


/// Read IMU
#[embassy_executor::task(pool_size = 1)]
pub async fn main_task(p: bsp::ImuPeripherals) {
    debug!("imu.main_task");
    let i2c = init_peripherals(p);

    let mut imu = Bno055::new(i2c);
    let id = match imu.id() {
        Ok(res) => {
            res
        },
        Err(_) => {
            error!("could not read device id");
            0xff
        }
    };
    info!("id = 0x{:02x}", id);

}

fn init_peripherals<'a>(p: bsp::ImuPeripherals) -> Twim<'a, bsp::ImuI2C> {
    let config = twim::Config::default();
    Twim::new(
        p.i2c,
        p.i2c_interrupt,
        p.i2c_sda_pin,
        p.i2c_scl_pin,
        config,
    )
}
