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

pub const QSPI_FLASH_SIZE: usize = 4*1024*1024;
pub const QSPI_FLASH_PAGE_SIZE: usize = 4096;

pub type Qspi = peripherals::QSPI;
pub type QspiInterrupt = interrupt::QSPI;
pub type QspiSckPin = peripherals::P0_19;
pub type QspiCsnPin = peripherals::P0_17;
pub type QspiIo0 = peripherals::P0_20;
pub type QspiIo1 = peripherals::P0_21;
pub type QspiIo2 = peripherals::P0_22;
pub type QspiIo3 = peripherals::P0_23;

pub struct QspiPeripherals {
    pub qspi: Qspi,
    pub interrupt: QspiInterrupt,
    pub sck: QspiSckPin,
    pub csn: QspiCsnPin,
    pub io0: QspiIo0,
    pub io1: QspiIo1,
    pub io2: QspiIo2,
    pub io3: QspiIo3,
}

pub fn init(p: Peripherals) -> (GpsPeripherals, ImuPeripherals, QspiPeripherals) {
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

    let irq = interrupt::take!(QSPI);
    let qspi = QspiPeripherals {
        qspi: p.QSPI,
        interrupt: irq,
        sck: p.P0_19,
        csn: p.P0_17,
        io0: p.P0_20,
        io1: p.P0_21,
        io2: p.P0_22,
        io3: p.P0_23,        
    };

    (gps, imu, qspi)
}
