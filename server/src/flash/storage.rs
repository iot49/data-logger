struct FlashStorage {
    
}

impl<'a> FlashStorage {
}


impl<'a> littlefs2::driver::Storage for FlashStorage {
    type CACHE_SIZE = ::littlefs2::consts::U256;
    type LOOKAHEAD_SIZE = ::littlefs2::consts::U32;

    const READ_SIZE: usize = 256;
    const WRITE_SIZE: usize = 256;
    const BLOCK_SIZE: usize = 4096;
    const BLOCK_COUNT: usize = 256;
    const BLOCK_CYCLES: isize = 500;

    fn read(&mut self, offset: usize, buf: &mut [u8]) -> littlefs2::io::Result<usize> {
        todo!()
    }

    fn write(&mut self, off: usize, data: &[u8]) -> littlefs2::io::Result<usize> {
        todo!()
    }

    fn erase(&mut self, off: usize, len: usize) -> littlefs2::io::Result<usize> {
        todo!()
    }

}
