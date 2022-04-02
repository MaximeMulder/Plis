const MEMORY_SIZE: usize = 0x100;

pub struct Memory {
    bytes: [u64; MEMORY_SIZE / 8],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            bytes: [0; MEMORY_SIZE / 8],
        }
    }

    pub fn get_8(&self, address: u64) -> u64 {
        let index = address as usize / 8;
        let offset = address as usize % 8;
        self.bytes[index] >> offset * 8
    }

    pub fn get_16(&self, address: u64) -> u64 {
        let index = address as usize / 4;
        let offset = address as usize % 4;
        self.bytes[index] >> offset * 16
    }

    pub fn get_32(&self, address: u64) -> u64 {
        let index = address as usize / 2;
        let offset = address as usize % 2;
        self.bytes[index] >> offset * 32
    }

    pub fn get_64(&self, address: u64) -> u64 {
        let index = address as usize;
        self.bytes[index]
    }

    pub fn set_8(&mut self, address: u64, value: u8) {
        let index = address as usize / 8;
        let offset = address as usize % 8;
        self.bytes[index] &= !(0xff << offset * 8);
        self.bytes[index] |= (value as u64) << offset * 8;
    }

    pub fn set_16(&mut self, address: u64, value: u16) {
        let index = address as usize / 4;
        let offset = address as usize % 4;
        self.bytes[index] &= !(0xffff << offset * 16);
        self.bytes[index] |= (value as u64) << offset * 16;
    }

    pub fn set_32(&mut self, address: u64, value: u32) {
        let index = address as usize / 2;
        let offset = address as usize % 2;
        self.bytes[index] &= !(0xffffffff << offset * 32);
        self.bytes[index] |= (value as u64) << offset * 32;
    }

    pub fn set_64(&mut self, address: u64, value: u64) {
        let index = address as usize;
        self.bytes[index] = value;
    }
}
