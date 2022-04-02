use architecture::Opcode;

use crate::program::Program;
use crate::register::RegisterId;

const THREADS_COUNT: usize = 16;

pub struct Threads {
    pub threads: [Thread; THREADS_COUNT],
}

impl Threads {
    pub fn new() -> Self {
        Self {
            threads: [(); THREADS_COUNT].map(|_| Thread::new()),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ThreadId(u8);

impl ThreadId {
    pub fn from_raw(raw: u8) -> Self {
        assert!((raw as usize) < THREADS_COUNT);
        Self(raw)
    }

    pub fn to_raw(self) -> usize {
        self.0 as usize
    }
}

pub struct Thread {
    pub active: bool,
    cursor: u64,
}

impl Thread {
    pub fn new() -> Self {
        Self {
            active: false,
            cursor: 0,
        }
    }

    pub fn next_opcode(&mut self, program: &Program) -> Opcode {
        let opcode = program.get_8(self.cursor);
        self.cursor += 1;
        Opcode::from_raw(opcode)
    }

    pub fn next_register(&mut self, program: &Program) -> RegisterId {
        let register = program.get_8(self.cursor);
        self.cursor += 1;
        RegisterId::from_raw(register)
    }

    pub fn next_thread(&mut self, program: &Program) -> ThreadId {
        let thread = program.get_8(self.cursor);
        self.cursor += 1;
        ThreadId::from_raw(thread)
    }

    pub fn next_const8(&mut self, program: &Program) -> u64 {
        let value = program.get_8(self.cursor);
        self.cursor += 1;
        value as u64
    }

    pub fn next_const16(&mut self, program: &Program) -> u64 {
        let value = program.get_16(self.cursor);
        self.cursor += 2;
        value as u64
    }

    pub fn next_const32(&mut self, program: &Program) -> u64 {
        let value = program.get_32(self.cursor);
        self.cursor += 4;
        value as u64
    }

    pub fn next_const64(&mut self, program: &Program) -> u64 {
        let value = program.get_64(self.cursor);
        self.cursor += 8;
        value
    }

    pub fn jump(&mut self, cursor: u64) {
        self.cursor = cursor;
    }
}
