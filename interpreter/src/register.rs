use architecture::REGISTERS_COUNT;

pub struct Registers {
    registers: [u64; REGISTERS_COUNT],
}

impl Registers {
    pub fn new() -> Self {
        Self {
            registers: [0; REGISTERS_COUNT],
        }
    }

    pub fn get(&self, id: RegisterId) -> u64 {
        self.registers[id.to_raw()]
    }

    pub fn set(&mut self, id: RegisterId, value: u64) {
        self.registers[id.to_raw()] = value;
    }
}

#[derive(Clone, Copy)]
pub struct RegisterId(u8);

impl RegisterId {
    pub fn from_raw(raw: u8) -> Self {
        assert!((raw as usize) < REGISTERS_COUNT);
        Self(raw)
    }

    pub fn to_raw(self) -> usize {
        self.0 as usize
    }
}
