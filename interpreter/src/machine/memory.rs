const MEMORY_SIZE: usize = 0x10000;

pub struct Memory {
    bytes: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            bytes: [0; MEMORY_SIZE],
        }
    }

    pub fn get_8(&self, address: u64) -> u8 {
        u8::from_ne_bytes(
            self.bytes.get(address as usize .. address as usize + 1).unwrap().try_into().unwrap()
        )
    }

    pub fn get_16(&self, address: u64) -> u16 {
        u16::from_ne_bytes(
            self.bytes.get(address as usize .. address as usize + 2).unwrap().try_into().unwrap()
        )
    }

    pub fn get_32(&self, address: u64) -> u32 {
        u32::from_ne_bytes(
            self.bytes.get(address as usize .. address as usize + 4).unwrap().try_into().unwrap()
        )
    }

    pub fn get_64(&self, address: u64) -> u64 {
        u64::from_ne_bytes(
            self.bytes.get(address as usize .. address as usize + 8).unwrap().try_into().unwrap()
        )
    }

    pub fn set_8(&mut self, address: u64, value: u8) {
        self.bytes.get_mut(address as usize .. address as usize + 1).unwrap().copy_from_slice(&value.to_ne_bytes());
    }

    pub fn set_16(&mut self, address: u64, value: u16) {
        self.bytes.get_mut(address as usize .. address as usize + 2).unwrap().copy_from_slice(&value.to_ne_bytes());
    }

    pub fn set_32(&mut self, address: u64, value: u32) {
        self.bytes.get_mut(address as usize .. address as usize + 4).unwrap().copy_from_slice(&value.to_ne_bytes());
    }

    pub fn set_64(&mut self, address: u64, value: u64) {
        self.bytes.get_mut(address as usize .. address as usize + 8).unwrap().copy_from_slice(&value.to_ne_bytes());
    }
}
