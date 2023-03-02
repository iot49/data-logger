use defmt::debug;
use embassy_nrf::{interrupt, peripherals, Peripherals};
use embassy_nrf::peripherals::{UARTE0, QSPI, TWISPI0};
use embassy_nrf::interrupt::InterruptExt;
use embassy_nrf::uarte::{self, Uarte};
use embassy_nrf::twim::{self, Twim};
use embassy_nrf::qspi::{self, Qspi};


/// Particle Xenon MX25L1606 4MByte Nor Flash
pub const QSPI_FLASH_SIZE: usize = 4 * 1024 * 1024;
pub const QSPI_FLASH_ALIGN: usize = 4;
pub const QSPI_FLASH_PAGE_SIZE: usize = 4096;

pub type GpsUart = Uarte<'static, UARTE0>;
pub type ImuI2C = Twim<'static, TWISPI0>;
pub type QspiFlash = Qspi<'static, QSPI, QSPI_FLASH_SIZE>;

pub struct IO {
    pub gps: GpsUart,
    pub imu: ImuI2C,
    pub flash: QspiFlash,
}


pub fn init(p: Peripherals) -> IO {
    debug!("board::particle_xenon init called");

    // gps uart
    let irq = interrupt::take!(UARTE0_UART0);
    irq.set_priority(interrupt::Priority::P3);
    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD9600;
    let gps = Uarte::new(p.UARTE0, irq, p.P0_08, p.P0_06, config);

    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
    irq.set_priority(interrupt::Priority::P3);
    let config = twim::Config::default();
    let imu = Twim::new(p.TWISPI0, irq, p.P0_26, p.P0_27, config);

    let irq = interrupt::take!(QSPI);
    irq.set_priority(interrupt::Priority::P3);
    let config = qspi::Config::default();
    let flash = Qspi::new(p.QSPI, irq, p.P0_19, p.P0_17, p.P0_20, p.P0_21, p.P0_22, p.P0_23, config);

    IO {
        gps: gps,
        imu: imu,
        flash: flash,
    }

}
