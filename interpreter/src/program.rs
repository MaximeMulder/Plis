pub struct Program {
    program: Box<[u8]>,
}

impl Program {
    pub fn new(program: Box<[u8]>) -> Self {
        Self {
            program,
        }
    }

    pub fn get(&self, cursor: u64) -> Option<u8> {
        self.program.get(cursor as usize).copied()
    }

    pub fn get_8(&self, cursor: u64) -> Option<u8> {
        Some(u8::from_be_bytes([
            self.get(cursor)?,
        ]))
    }

    pub fn get_16(&self, cursor: u64) -> Option<u16> {
        Some(u16::from_be_bytes([
            self.get(cursor + 0)?,
            self.get(cursor + 1)?,
        ]))
    }

    pub fn get_32(&self, cursor: u64) -> Option<u32> {
        Some(u32::from_be_bytes([
            self.get(cursor + 0)?,
            self.get(cursor + 1)?,
            self.get(cursor + 2)?,
            self.get(cursor + 3)?,
        ]))
    }

    pub fn get_64(&self, cursor: u64) -> Option<u64> {
        Some(u64::from_be_bytes([
            self.get(cursor + 0)?,
            self.get(cursor + 1)?,
            self.get(cursor + 2)?,
            self.get(cursor + 3)?,
            self.get(cursor + 4)?,
            self.get(cursor + 5)?,
            self.get(cursor + 6)?,
            self.get(cursor + 7)?,
        ]))
    }
}
