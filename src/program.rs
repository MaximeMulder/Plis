pub struct Program {
    program: Box<[u8]>,
}

impl Program {
    pub fn new(program: Box<[u8]>) -> Self {
        Self {
            program,
        }
    }

    pub fn get_8(&self, cursor: u64) -> u8 {
        self.program[cursor as usize]
    }

    pub fn get_16(&self, cursor: u64) -> u16 {
        let high = self.get_8(cursor) as u16;
        let low = self.get_8(cursor + 1) as u16;
        high << 8 | low
    }

    pub fn get_32(&self, cursor: u64) -> u32 {
        let high = self.get_16(cursor) as u32;
        let low = self.get_16(cursor + 2) as u32;
        high << 16 | low
    }

    pub fn get_64(&self, cursor: u64) -> u64 {
        let high = self.get_32(cursor) as u64;
        let low = self.get_32(cursor + 4) as u64;
        high << 32 | low
    }
}
