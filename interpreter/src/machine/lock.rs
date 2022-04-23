use architecture::LOCKS_COUNT;

use crate::machine::Machine;

pub struct Locks {
    locks: [Lock; LOCKS_COUNT],
}

impl Locks {
    pub fn new() -> Self {
        Self {
            locks: [(); LOCKS_COUNT].map(|_| Lock::new()),
        }
    }

    pub fn get(&self, id: LockId) -> &Lock {
        &self.locks[LockId::to_raw(id)]
    }

    pub fn get_mut(&mut self, id: LockId) -> &mut Lock {
        &mut self.locks[LockId::to_raw(id)]
    }
}

pub struct Lock {
    locked: bool,
}

impl Lock {
    pub fn new() -> Self {
        Self {
            locked: true,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LockId(u8);

impl LockId {
    pub fn from_raw(raw: u8) -> Option<Self> {
        ((raw as usize) < LOCKS_COUNT).then(|| Self(raw))
    }

    pub fn to_raw(id: LockId) -> usize {
        id.0 as usize
    }
}

impl Machine<'_> {
    pub fn locked(&self, lock_id: LockId) -> bool {
        self.locks.get(lock_id).locked
    }

    pub fn lock(&mut self, lock_id: LockId) {
        self.locks.get_mut(lock_id).locked = true;
    }

    pub fn unlock(&mut self, lock_id: LockId) {
        self.locks.get_mut(lock_id).locked = false;
        for thread in self.threads.get_threads().into_iter().copied() {
            let thread = self.threads.get_mut(thread);
            if thread.is_waiting(lock_id) {
                thread.start();
            }
        }
    }
}
