use defmt::{unwrap, info};
use crate::bsp;
use super::comm::Comm;
use embassy_nrf::qspi::{Qspi, Config};
use embassy_nrf::peripherals;

mod storage;
mod history;
mod file_system;

const FLASH_SIZE: usize = bsp::QSPI_FLASH_SIZE;
const PAGE_SIZE: usize = bsp::QSPI_FLASH_PAGE_SIZE;

// Workaround for alignment requirements.
// Nicer API will probably come in the future.
#[repr(C, align(4))]
struct AlignedBuf([u8; PAGE_SIZE]);


#[embassy_executor::task]
pub async fn main_task(comm: &'static Comm, mut qspi: bsp::QspiFlash) {

    // Read chip id
    let mut id = [1; 3];
    unwrap!(qspi.custom_instruction(0x9F, &[], &mut id).await);
    info!("id: {:x}", id);

    // Xenon 4MBit chip id
    assert_eq!(id, [194u8, 32, 22]);

    // Read status register
    let mut status = [4; 1];
    unwrap!(qspi.custom_instruction(0x05, &[], &mut status).await);

    info!("status: 0x{:x}", status[0]);

    if status[0] & 0x40 == 0 {
        status[0] |= 0x40;
        unwrap!(qspi.custom_instruction(0x01, &status, &mut []).await);
        info!("enabled quad in status");
    }

    let mut buf = AlignedBuf([0u8; bsp::QSPI_FLASH_PAGE_SIZE]);

    let pattern = |a: u32| (a ^ (a >> 8) ^ (a >> 16) ^ (a >> 24)) as u8;

    for i in 0..8 {
        info!("page {:?}: erasing... ", i);
        unwrap!(qspi.erase(i * PAGE_SIZE).await);

        for j in 0..PAGE_SIZE {
            buf.0[j] = pattern((j + i * PAGE_SIZE) as u32);
        }

        info!("programming...");
        unwrap!(qspi.write(i * PAGE_SIZE, &buf.0).await);
    }

    for i in 0..8 {
        info!("page {:?}: reading... ", i);
        unwrap!(qspi.read(i * PAGE_SIZE, &mut buf.0).await);

        info!("verifying...");
        for j in 0..PAGE_SIZE {
            assert_eq!(buf.0[j], pattern((j + i * PAGE_SIZE) as u32));
        }
    }

    info!("small reads and writes");

    unwrap!(qspi.erase(0).await);
    unwrap!(qspi.read(0, &mut buf.0[..256]).await);
    info!("after erase: {}", &buf.0[..256]);
    let _ = &buf.0[..16].copy_from_slice(b"0123456789012345");
    unwrap!(qspi.write(8, &buf.0[..4]).await);
    unwrap!(qspi.read(8, &mut buf.0[..4]).await);
    info!("after write: {}", &buf.0[..4]);
   
}
