use architecture::{ Opcode, THREADS_COUNT };

use crate::lock::LockId;
use crate::program::Program;
use crate::register::RegisterId;

pub struct Threads {
    threads: [Thread; THREADS_COUNT],
}

impl Threads {
    pub fn new() -> Self {
        Self {
            threads: [(); THREADS_COUNT].map(|_| Thread::new()),
        }
    }

    pub fn get(&self, id: ThreadId) -> &Thread {
        &self.threads[ThreadId::to_raw(id)]
    }

    pub fn get_mut(&mut self, id: ThreadId) -> &mut Thread {
        &mut self.threads[ThreadId::to_raw(id)]
    }

    pub fn iterate(&self) -> impl Iterator<Item = ThreadId> {
        (0 as u8 .. THREADS_COUNT as u8).map(|i| ThreadId::from_raw(i))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThreadStatus {
    Active,
    Inactive,
    Waiting(LockId),
}

pub struct Thread {
    active: ThreadStatus,
    cursor: u64,
}

impl Thread {
    pub fn new() -> Self {
        Self {
            active: ThreadStatus::Inactive,
            cursor: 0,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active == ThreadStatus::Active
    }

    pub fn is_waiting(&self, other: LockId) -> bool {
        if let ThreadStatus::Waiting(lock) = self.active {
            lock == other
        } else {
            false
        }
    }

    pub fn jump(&mut self, cursor: u64) {
        self.cursor = cursor;
    }

    pub fn start(&mut self) {
        self.active = ThreadStatus::Active;
    }

    pub fn stop(&mut self) {
        self.active = ThreadStatus::Inactive;
    }

    pub fn wait(&mut self, lock: LockId) {
        self.active = ThreadStatus::Waiting(lock);
    }
}

impl Thread {
    pub fn next_opcode(&mut self, program: &Program) -> Opcode {
        let opcode = program.get_8(self.cursor);
        self.cursor += 1;
        Opcode::from_raw(opcode).unwrap()
    }

    pub fn next_register(&mut self, program: &Program) -> RegisterId {
        let register = program.get_8(self.cursor);
        self.cursor += 1;
        RegisterId::from_raw(register)
    }

    pub fn next_lock(&mut self, program: &Program) -> LockId {
        let lock = program.get_8(self.cursor);
        self.cursor += 1;
        LockId::from_raw(lock)
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
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ThreadId(u8);

impl ThreadId {
    pub fn from_raw(raw: u8) -> Self {
        assert!((raw as usize) < THREADS_COUNT);
        Self(raw)
    }

    pub fn to_raw(id: ThreadId) -> usize {
        id.0 as usize
    }
}
