use embassy_nrf::twim::{self, Twim};
use embassy_nrf::gpio::{Level, Output, OutputDrive};

pub fn setup () {
    // imu
    info!("Initializing TWI...");
    let config = twim::Config::default();
    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
    let imu_sda = p.P0_07;
    let imu_scl = p.P0_27;
    let mut imu_twi = Twim::new(p.TWISPI0, irq, imu_sda, imu_scl, config);
    let mut imu_pwr = Output::new(p.P1_08, Level::High, OutputDrive::HighDrive);
    let imu_address = 0x6au8;

    imu_pwr.set_high();

}