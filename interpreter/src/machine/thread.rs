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
            threads: (0 .. THREADS_COUNT).map(|i| Thread::new(ThreadId::from_raw(i as u8).unwrap())).collect(),
        }
    }

    pub fn get(&self, id: ThreadId) -> &Thread {
        &self.threads[ThreadId::to_raw(id)]
    }

    pub fn get_mut(&mut self, id: ThreadId) -> &mut Thread {
        &mut self.threads[ThreadId::to_raw(id)]
    }

    pub fn get_threads(&self) -> Box<[ThreadId]> {
        self.threads.iter()
            .map(|thread| thread.id)
            .collect()
    }

    pub fn get_actives(&self) -> Box<[ThreadId]> {
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ThreadId(u8);

impl ThreadId {
    pub fn from_raw(raw: u8) -> Option<Self> {
        ((raw as usize) < THREADS_COUNT).then(|| Self(raw))
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

impl Machine<'_> {
    pub fn get_8(&mut self, thread_id: ThreadId) -> u8 {
        let cursor = self.threads.get(thread_id).cursor;
        let value = self.program.get_8(cursor).unwrap_or_else(|| self.error_program_address(thread_id, cursor));
        self.threads.get_mut(thread_id).cursor += 1;
        value
    }

    pub fn get_16(&mut self, thread_id: ThreadId) -> u16 {
        let cursor = self.threads.get(thread_id).cursor;
        let value = self.program.get_16(cursor).unwrap_or_else(|| self.error_program_address(thread_id, cursor));
        self.threads.get_mut(thread_id).cursor += 2;
        value
    }

    pub fn get_32(&mut self, thread_id: ThreadId) -> u32 {
        let cursor = self.threads.get(thread_id).cursor;
        let value = self.program.get_32(cursor).unwrap_or_else(|| self.error_program_address(thread_id, cursor));
        self.threads.get_mut(thread_id).cursor += 4;
        value
    }

    pub fn get_64(&mut self, thread_id: ThreadId) -> u64 {
        let cursor = self.threads.get(thread_id).cursor;
        let value = self.program.get_64(cursor).unwrap_or_else(|| self.error_program_address(thread_id, cursor));
        self.threads.get_mut(thread_id).cursor += 8;
        value
    }
}

impl Machine<'_> {
    pub fn next_opcode(&mut self, thread_id: ThreadId) -> Opcode {
        let value = self.get_8(thread_id);
        Opcode::from_raw(value).unwrap_or_else(|| self.error_invalid_opcode(thread_id, value))
    }

    pub fn next_register(&mut self, thread_id: ThreadId) -> RegisterId {
        let value = self.get_8(thread_id);
        RegisterId::from_raw(value).unwrap_or_else(|| self.error_invalid_register(thread_id, value))
    }

    pub fn next_lock(&mut self, thread_id: ThreadId) -> LockId {
        let value = self.get_8(thread_id);
        LockId::from_raw(value).unwrap_or_else(|| self.error_invalid_lock(thread_id, value))
    }

    pub fn next_thread(&mut self, thread_id: ThreadId) -> ThreadId {
        let value = self.get_8(thread_id);
        ThreadId::from_raw(value).unwrap_or_else(|| self.error_invalid_thread(thread_id, value))
    }

    pub fn next_const8(&mut self, thread_id: ThreadId) -> u64 {
        self.get_8(thread_id) as u64
    }

    pub fn next_const16(&mut self, thread_id: ThreadId) -> u64 {
        self.get_16(thread_id) as u64
    }

    pub fn next_const32(&mut self, thread_id: ThreadId) -> u64 {
        self.get_32(thread_id) as u64
    }

    pub fn next_const64(&mut self, thread_id: ThreadId) -> u64 {
        self.get_64(thread_id) as u64
    }
}
