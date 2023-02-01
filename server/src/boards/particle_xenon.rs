use defmt::debug;
use embassy_nrf::{interrupt, peripherals, Peripherals};
use embassy_nrf::interrupt::InterruptExt;

pub type GpsUarte = peripherals::UARTE0;
pub type GpsUarteInterrupt = interrupt::UARTE0_UART0;
pub type GpsUarteRxPin = peripherals::P0_08;
pub type GpsUarteTxPin = peripherals::P0_06;

pub struct GpsPeripherals {
    pub uarte: GpsUarte,
    pub uarte_interrupt: GpsUarteInterrupt,
    pub uarte_rx_pin: GpsUarteRxPin,
    pub uarte_tx_pin: GpsUarteTxPin,
}

pub type ImuI2C = peripherals::TWISPI0;
pub type ImuI2CInterrupt = interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0;
pub type ImuI2CSdaPin = peripherals::P0_26;
pub type ImuI2CSclPin = peripherals::P0_27;

pub struct ImuPeripherals {
    pub i2c: ImuI2C,
    pub i2c_interrupt: ImuI2CInterrupt,
    pub i2c_sda_pin: ImuI2CSdaPin,
    pub i2c_scl_pin: ImuI2CSclPin,
}


pub fn init(p: Peripherals) -> (GpsPeripherals, ImuPeripherals) {
    debug!("board::particle_xenon init called");

    let irq = interrupt::take!(UARTE0_UART0);
    irq.set_priority(interrupt::Priority::P3);
    let gps = GpsPeripherals {
        uarte: p.UARTE0,
        uarte_interrupt: irq,
        uarte_rx_pin: p.P0_08,
        uarte_tx_pin: p.P0_06,
    };

    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
    irq.set_priority(interrupt::Priority::P3);
    let imu = ImuPeripherals {
        i2c: p.TWISPI0,
        i2c_interrupt: irq,
        i2c_sda_pin: p.P0_26,
        i2c_scl_pin: p.P0_27,
    };

    (gps, imu)
}
