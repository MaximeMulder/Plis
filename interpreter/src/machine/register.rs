use std::fmt::{ Display, Formatter };

use architecture::REGISTERS_COUNT;

use crate::machine::Machine;

pub struct Registers {
    registers: Box<[Register]>,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            registers: (0 .. REGISTERS_COUNT).map(|_| Register::new()).collect(),
        }
    }

    pub fn reset(&mut self) {
        for register in self.registers.iter_mut() {
            register.status = RegisterStatus::None;
        }
    }

    fn get(&self, id: RegisterId) -> &Register {
        &self.registers[RegisterId::to_raw(id)]
    }

    fn get_mut(&mut self, id: RegisterId) -> &mut Register {
        &mut self.registers[RegisterId::to_raw(id)]
    }
}

#[derive(PartialEq, Eq)]
pub enum RegisterStatus {
    None,
    Read,
    Write,
}

pub struct Register {
    status: RegisterStatus,
    value: u64,
}

impl Register {
    pub fn new() -> Self {
        Self {
            status: RegisterStatus::None,
            value: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct RegisterId(u8);

impl RegisterId {
    pub fn from_raw(raw: u8) -> Option<Self> {
        ((raw as usize) < REGISTERS_COUNT).then(|| Self(raw))
    }

    pub fn to_raw(self) -> usize {
        self.0 as usize
    }
}

impl Display for RegisterId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("r")?;
        formatter.write_fmt(format_args!("{}", self.0))?;
        Ok(())
    }
}

impl Machine<'_> {
    pub fn register_read(&mut self, register_id: RegisterId) -> u64 {
        let register = self.registers.get_mut(register_id);
        if register.status == RegisterStatus::Write {
            self.error_data_race(register_id);
        }

        register.status = RegisterStatus::Read;
        register.value
    }

    pub fn register_write(&mut self, register_id: RegisterId, value: u64) {
        let register = self.registers.get_mut(register_id);
        if register.status != RegisterStatus::None {
            self.error_data_race(register_id);
        }

        register.status = RegisterStatus::Write;
        register.value = value;
    }
}
