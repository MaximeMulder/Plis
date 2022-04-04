const LOCKS_COUNT: usize = 64;

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

#[derive(Clone, Copy)]
pub struct LockId(u8);

impl LockId {
    pub fn from_raw(raw: u8) -> Self {
        assert!((raw as usize) < LOCKS_COUNT);
        Self(raw)
    }

    pub fn to_raw(id: LockId) -> usize {
        id.0 as usize
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

    pub fn locked(&self) -> bool {
        self.locked
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }
}
