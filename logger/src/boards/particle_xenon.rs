use defmt::debug;
use embassy_nrf::{interrupt, peripherals, Peripherals};

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


pub fn init(p: Peripherals) -> GpsPeripherals {
    debug!("board::particle_xenon init called");
    GpsPeripherals {
        uarte: p.UARTE0,
        uarte_interrupt: interrupt::take!(UARTE0_UART0),
        uarte_rx_pin: p.P0_08,
        uarte_tx_pin: p.P0_06,
    }
}
