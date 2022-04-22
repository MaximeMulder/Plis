use std::process::exit;

pub struct Program {
    program: Box<[u8]>,
}

impl Program {
    pub fn new(program: Box<[u8]>) -> Self {
        Self {
            program,
        }
    }

    pub fn get(&self, cursor: u64) -> u8 {
        self.program.get(cursor as usize).copied().unwrap_or_else(|| self.error_program("Program cursor out of bounds."))
    }

    pub fn get_8(&self, cursor: u64) -> u8 {
        u8::from_be_bytes([
            self.get(cursor),
        ])
    }

    pub fn get_16(&self, cursor: u64) -> u16 {
        u16::from_be_bytes([
            self.get(cursor + 0),
            self.get(cursor + 1),
        ])
    }

    pub fn get_32(&self, cursor: u64) -> u32 {
        u32::from_be_bytes([
            self.get(cursor + 0),
            self.get(cursor + 1),
            self.get(cursor + 2),
            self.get(cursor + 3),
        ])
    }

    pub fn get_64(&self, cursor: u64) -> u64 {
        u64::from_be_bytes([
            self.get(cursor + 0),
            self.get(cursor + 1),
            self.get(cursor + 2),
            self.get(cursor + 3),
            self.get(cursor + 4),
            self.get(cursor + 5),
            self.get(cursor + 6),
            self.get(cursor + 7),
        ])
    }

    fn error_program(&self, message: &str) -> ! {
        eprintln!("ERROR: {}", message);
        exit(0);
    }
}
