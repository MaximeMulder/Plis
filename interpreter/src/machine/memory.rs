use std::ops::Range;

use crate::machine::Machine;
use crate::machine::thread::ThreadId;

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
}

impl Machine<'_> {
    pub fn load8(&self, thread_id: ThreadId, address: u64) -> u8 {
        u8::from_ne_bytes(self.load_x(thread_id, address, 1).try_into().unwrap())
    }

    pub fn load16(&self, thread_id: ThreadId, address: u64) -> u16 {
        u16::from_ne_bytes(self.load_x(thread_id, address, 2).try_into().unwrap())
    }

    pub fn load32(&self, thread_id: ThreadId, address: u64) -> u32 {
        u32::from_ne_bytes(self.load_x(thread_id, address, 4).try_into().unwrap())
    }

    pub fn load64(&self, thread_id: ThreadId, address: u64) -> u64 {
        u64::from_ne_bytes(self.load_x(thread_id, address, 8).try_into().unwrap())
    }

    pub fn store8(&mut self, thread_id: ThreadId, address: u64, value: u8) {
        self.store_x(thread_id, address, 1, &value.to_ne_bytes());
    }

    pub fn store16(&mut self, thread_id: ThreadId, address: u64, value: u16) {
        self.store_x(thread_id, address, 2, &value.to_ne_bytes());
    }

    pub fn store32(&mut self, thread_id: ThreadId, address: u64, value: u32) {
        self.store_x(thread_id, address, 4, &value.to_ne_bytes());
    }

    pub fn store64(&mut self, thread_id: ThreadId, address: u64, value: u64) {
        self.store_x(thread_id, address, 8, &value.to_ne_bytes());
    }
}

impl Machine<'_> {
    fn load_x(&self, thread_id: ThreadId, address: u64, length: usize) -> &[u8] {
        self.memory.bytes.get(Self::get_range(address, length))
            .unwrap_or_else(|| self.error_memory_address(thread_id, address))
    }

    fn store_x(&mut self, thread_id: ThreadId, address: u64, length: usize, value: &[u8]) {
        match self.memory.bytes.get_mut(Self::get_range(address, length)) {
            Some(slice) => slice.copy_from_slice(value),
            None => self.error_memory_address(thread_id, address),
        }
    }

    fn get_range(address: u64, length: usize) -> Range<usize> {
        address as usize .. address as usize + length
    }
}
