struct FlashStorage {
    
}

impl<'a> FlashStorage {
}


impl<'a> littlefs2::driver::Storage for FlashStorage {
    type CACHE_SIZE = ::littlefs2::consts::U256;
    type LOOKAHEAD_SIZE = ::littlefs2::consts::U32;

    /// smallest unit that can be read
    const READ_SIZE: usize = 4;
    /// smalles unit that can be written
    const WRITE_SIZE: usize = 4;
    const BLOCK_SIZE: usize = 4096;
    /// BLOCK_COUNT * BLOCK_SIZE = capacity
    const BLOCK_COUNT: usize = 256;
    /// 100 (good wear leveling)... 1000 (high performance), -1 disables wear-leveling
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
