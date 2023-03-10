use defmt::debug;
use embassy_nrf::{interrupt, peripherals, Peripherals};
use embassy_nrf::interrupt::InterruptExt;

pub type GpsUarte = peripherals::UARTE0;
pub type GpsUarteInterrupt = interrupt::UARTE0_UART0;
pub type GpsUarteRxPin = peripherals::P0_04;
pub type GpsUarteTxPin = peripherals::P0_02;

pub struct GpsPeripherals {
    pub uarte: GpsUarte,
    pub uarte_interrupt: GpsUarteInterrupt,
    pub uarte_rx_pin: GpsUarteRxPin,
    pub uarte_tx_pin: GpsUarteTxPin,
}

pub fn init(p: Peripherals) -> GpsPeripherals {
    debug!("board::microbit_v2 init called");
    let irq = interrupt::take!(UARTE0_UART0);
    irq.set_priority(interrupt::Priority::P3);
    GpsPeripherals {
        uarte: p.UARTE0,
        uarte_interrupt: irq,
        uarte_rx_pin: p.P0_04,
        uarte_tx_pin: p.P0_02,
    }
}
