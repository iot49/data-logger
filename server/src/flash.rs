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

static mut BUF: AlignedBuf = AlignedBuf([0; 4096]);

struct Flash<'a> {
    qspi: Qspi<'a, peripherals::QSPI, FLASH_SIZE>,
}

// impl<'a> ekv::flash::Flash for Flash<'a> {
impl<'a> Flash<'a> {
    // type Error = Infallible;

    fn page_count(&self) -> usize {
        bsp::QSPI_FLASH_MAX_PAGE_COUNT
    }

    async fn erase(&mut self, page_id: usize) {
        self.qspi.erase(page_id * bsp::QSPI_FLASH_PAGE_SIZE).await.unwrap();
    }

    /// double buffering ... is this needed?
    async fn read(&mut self, page_id: usize, offset: usize, data: &mut [u8]) {
        let address = page_id * bsp::QSPI_FLASH_PAGE_SIZE + offset;
        unsafe {
            self.qspi.read(address, &mut BUF.0[..data.len()]).await.unwrap();
            data.copy_from_slice(&BUF.0[..data.len()]);
        }
    }

    async fn write(&mut self, page_id: usize, offset: usize, data: &[u8]) {
        let address = page_id * bsp::QSPI_FLASH_PAGE_SIZE + offset;
        unsafe {
            BUF.0[..data.len()].copy_from_slice(data);
            self.qspi.write(address, &BUF.0[..data.len()]).await.unwrap();
        }
    }
}


#[embassy_executor::task]
pub async fn main_task(comm: &'static Comm, p: bsp::QspiPeripherals) {
    let config = Config::default();
    let mut q: Qspi<_, FLASH_SIZE> = Qspi::new(
        p.qspi, p.interrupt, p.sck, p.csn, p.io0, p.io1, p.io2, p.io3, config
    );

    // Read chip id
    let mut id = [1; 3];
    unwrap!(q.custom_instruction(0x9F, &[], &mut id).await);
    info!("id: {:x}", id);

    // Xenon 4MBit chip id
    assert_eq!(id, [194u8, 32, 22]);

    // Read status register
    let mut status = [4; 1];
    unwrap!(q.custom_instruction(0x05, &[], &mut status).await);

    info!("status: 0x{:x}", status[0]);

    if status[0] & 0x40 == 0 {
        status[0] |= 0x40;
        unwrap!(q.custom_instruction(0x01, &status, &mut []).await);
        info!("enabled quad in status");
    }
    
    let mut buf = AlignedBuf([0u8; PAGE_SIZE]);

    let pattern = |a: u32| (a ^ (a >> 8) ^ (a >> 16) ^ (a >> 24)) as u8;

    for i in 0..8 {
        info!("page {:?}: erasing... ", i);
        unwrap!(q.erase(i * PAGE_SIZE).await);

        for j in 0..PAGE_SIZE {
            buf.0[j] = pattern((j + i * PAGE_SIZE) as u32);
        }

        info!("programming...");
        unwrap!(q.write(i * PAGE_SIZE, &buf.0).await);
    }

    for i in 0..8 {
        info!("page {:?}: reading... ", i);
        unwrap!(q.read(i * PAGE_SIZE, &mut buf.0).await);

        info!("verifying...");
        for j in 0..PAGE_SIZE {
            assert_eq!(buf.0[j], pattern((j + i * PAGE_SIZE) as u32));
        }
    }

    info!("small reads and writes");

    unwrap!(q.erase(0).await);
    unwrap!(q.read(0, &mut buf.0[..256]).await);
    info!("after erase: {}", &buf.0[..256]);
    let _ = &buf.0[..16].copy_from_slice(b"0123456789012345");
    unwrap!(q.write(8, &buf.0[..4]).await);
    unwrap!(q.read(8, &mut buf.0[..4]).await);
    info!("after write: {}", &buf.0[..4]);

    // now with driver ...
    let mut flash = Flash { qspi: q };

    const PAGE: usize = 7;
    const OFFSET: usize = 28;

    flash.erase(PAGE).await;
    flash.write(PAGE, OFFSET, b"01234567").await;
    let mut buf = [0u8; 16];
    flash.read(PAGE, OFFSET, &mut buf).await;

    info!("----------- read {:?}", buf);

   
}
