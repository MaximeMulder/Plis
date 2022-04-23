use std::fmt::{ Display, Formatter };

use architecture::{ Opcode, THREADS_COUNT };

use crate::machine::lock::LockId;
use crate::machine::Machine;
use crate::machine::register::RegisterId;

pub struct Threads {
    threads: Box<[Thread]>,
}

impl Threads {
    pub fn new() -> Self {
        Self {
            threads: (0 .. THREADS_COUNT).map(|i| Thread::new(ThreadId::from_raw(i as u8))).collect(),
        }
    }

    pub fn get(&self, id: ThreadId) -> &Thread {
        &self.threads[ThreadId::to_raw(id)]
    }

    pub fn get_mut(&mut self, id: ThreadId) -> &mut Thread {
        &mut self.threads[ThreadId::to_raw(id)]
    }

    pub fn actives(&self) -> Box<[ThreadId]> {
        self.threads.iter()
            .filter(|thread| thread.is_active())
            .map(|thread| thread.id)
            .collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThreadStatus {
    Active,
    Inactive,
    Waiting(LockId),
}

pub struct Thread {
    id: ThreadId,
    cursor: u64,
    active: ThreadStatus,
}

impl Thread {
    pub fn new(id: ThreadId) -> Self {
        Self {
            id,
            cursor: 0,
            active: ThreadStatus::Inactive,
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

    pub fn id(&self) -> ThreadId {
        self.id
    }

    pub fn cursor(&self) -> u64 {
        self.cursor
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
}

impl Machine<'_> {
    pub fn next_opcode(&mut self, thread: ThreadId) -> Opcode {
        let thread = self.threads.get_mut(thread);
        let value = self.program.get_8(thread.cursor);
        thread.cursor += 1;
        Opcode::from_raw(value).unwrap()
    }

    pub fn next_register(&mut self, thread: ThreadId) -> RegisterId {
        let thread = self.threads.get_mut(thread);
        let value = self.program.get_8(thread.cursor);
        thread.cursor += 1;
        RegisterId::from_raw(value)
    }

    pub fn next_lock(&mut self, thread: ThreadId) -> LockId {
        let thread = self.threads.get_mut(thread);
        let value = self.program.get_8(thread.cursor);
        thread.cursor += 1;
        LockId::from_raw(value)
    }

    pub fn next_thread(&mut self, thread: ThreadId) -> ThreadId {
        let thread = self.threads.get_mut(thread);
        let value = self.program.get_8(thread.cursor);
        thread.cursor += 1;
        ThreadId::from_raw(value)
    }

    pub fn next_const8(&mut self, thread: ThreadId) -> u64 {
        let thread = self.threads.get_mut(thread);
        let value = self.program.get_8(thread.cursor);
        thread.cursor += 1;
        value as u64
    }

    pub fn next_const16(&mut self, thread: ThreadId) -> u64 {
        let thread = self.threads.get_mut(thread);
        let value = self.program.get_16(thread.cursor);
        thread.cursor += 2;
        value as u64
    }

    pub fn next_const32(&mut self, thread: ThreadId) -> u64 {
        let thread = self.threads.get_mut(thread);
        let value = self.program.get_32(thread.cursor);
        thread.cursor += 4;
        value as u64
    }

    pub fn next_const64(&mut self, thread: ThreadId) -> u64 {
        let thread = self.threads.get_mut(thread);
        let value = self.program.get_64(thread.cursor);
        thread.cursor += 8;
        value as u64
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

impl Display for ThreadId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("t")?;
        formatter.write_fmt(format_args!("{}", self.0))?;
        Ok(())
    }
}