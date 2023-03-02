#![allow(dead_code)]
use crate::bsp;

use defmt::*;
//use embassy_time::{Duration, Timer};

//use bno055::Bno055;
//use i2cdev_bno055::*;

/// Read IMU
#[embassy_executor::task(pool_size = 1)]
pub async fn main_task(i2c: bsp::ImuI2C) {
    debug!("imu.main_task");

    /* 
    if false {
        let mut imu = Bno055::new(i2c);
        imu.with_alternative_address();

        loop {
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
            Timer::after(Duration::from_millis(100));
        }
    }
    */
}

