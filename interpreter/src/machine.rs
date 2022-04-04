use std::io::stdin;

use architecture::Opcode;

use crate::lock::Locks;
use crate::memory::Memory;
use crate::program::Program;
use crate::register::{Registers, RegisterId};
use crate::thread::{Threads, ThreadId};

pub struct Machine {
    threads: Threads,
    registers: Registers,
    locks: Locks,
    memory: Memory,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            threads: Threads::new(),
            registers: Registers::new(),
            locks: Locks::new(),
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self, program: &Program) {
        self.threads.get_mut(ThreadId::from_raw(0)).active = true;
        loop {
            for id in self.threads.iter_ids() {
                let thread = self.threads.get(id);
                if !thread.active {
                    continue;
                }

                let opcode = self.threads.get_mut(id).next_opcode(program);
                match opcode {
                    Opcode::Nop => {},
                    Opcode::Const8 => {
                        let thread = self.threads.get_mut(id);
                        let register = thread.next_register(program);
                        let constant = thread.next_const8(program);
                        self.registers.set(register, constant);
                    },
                    Opcode::Const16 => {
                        let thread = self.threads.get_mut(id);
                        let register = thread.next_register(program);
                        let constant = thread.next_const16(program);
                        self.registers.set(register, constant);
                    },
                    Opcode::Const32 => {
                        let thread = self.threads.get_mut(id);
                        let register = thread.next_register(program);
                        let constant = thread.next_const32(program);
                        self.registers.set(register, constant);
                    },
                    Opcode::Const64 => {
                        let thread = self.threads.get_mut(id);
                        let register = thread.next_register(program);
                        let constant = thread.next_const64(program);
                        self.registers.set(register, constant);
                    },
                    Opcode::Load8 => {
                        let thread = self.threads.get_mut(id);
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        let address = self.registers.get(source);
                        let value = self.memory.get_8(address);
                        self.registers.set(destination, value);
                    },
                    Opcode::Load16 => {
                        let thread = self.threads.get_mut(id);
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        let address = self.registers.get(source);
                        let value = self.memory.get_16(address);
                        self.registers.set(destination, value);
                    },
                    Opcode::Load32 => {
                        let thread = self.threads.get_mut(id);
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        let address = self.registers.get(source);
                        let value = self.memory.get_32(address);
                        self.registers.set(destination, value);
                    },
                    Opcode::Load64 => {
                        let thread = self.threads.get_mut(id);
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        let value = self.memory.get_64(self.registers.get(source));
                        self.registers.set(destination, value);
                    },
                    Opcode::Store8 => {
                        let thread = self.threads.get_mut(id);
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        let address = self.registers.get(destination);
                        let value = self.registers.get(source) as u8;
                        self.memory.set_8(address, value);
                    },
                    Opcode::Store16 => {
                        let thread = self.threads.get_mut(id);
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        let address = self.registers.get(destination);
                        let value = self.registers.get(source) as u16;
                        self.memory.set_16(address, value);
                    },
                    Opcode::Store32 => {
                        let thread = self.threads.get_mut(id);
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        let address = self.registers.get(destination);
                        let value = self.registers.get(source) as u32;
                        self.memory.set_32(address, value);
                    },
                    Opcode::Store64 => {
                        let thread = self.threads.get_mut(id);
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        let address = self.registers.get(destination);
                        let value = self.registers.get(source);
                        self.memory.set_64(address, value);
                    },
                    Opcode::And => {
                        let thread = self.threads.get_mut(id);
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        self.registers.set(result, a & b);
                    },
                    Opcode::Or => {
                        let thread = self.threads.get_mut(id);
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        self.registers.set(result, a | b);
                    },
                    Opcode::Xor => {
                        let thread = self.threads.get_mut(id);
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        self.registers.set(result, a ^ b);
                    },
                    Opcode::ShiftL => {
                        let thread = self.threads.get_mut(id);
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        self.registers.set(result, a << b);
                    },
                    Opcode::ShiftR => {
                        let thread = self.threads.get_mut(id);
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        self.registers.set(result, a >> b);
                    },
                    Opcode::Add => {
                        let thread = self.threads.get_mut(id);
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        self.registers.set(result, a + b);
                    },
                    Opcode::Sub => {
                        let thread = self.threads.get_mut(id);
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        self.registers.set(result, a - b);
                    },
                    Opcode::Mul => {
                        let thread = self.threads.get_mut(id);
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        self.registers.set(result, a * b);
                    },
                    Opcode::Div => {
                        let thread = self.threads.get_mut(id);
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        self.registers.set(result, a / b);
                    },
                    Opcode::Rem => {
                        let thread = self.threads.get_mut(id);
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        let _lock = thread.next_lock(program);
                        self.registers.set(result, a % b);
                    },
                    Opcode::Jump => {
                        let thread = self.threads.get_mut(id);
                        let address = self.registers.get(thread.next_register(program));
                        thread.jump(address);
                    },
                    Opcode::JumpIf => {
                        let thread = self.threads.get_mut(id);
                        let address = self.registers.get(thread.next_register(program));
                        let condition = self.registers.get(thread.next_register(program));
                        if condition != 0 {
                            thread.jump(address);
                        }
                    },
                    Opcode::Wait => {
                        let thread = self.threads.get_mut(id);
                        let lock = self.locks.get(thread.next_lock(program));
                        if lock.locked() {
                            thread.wait();
                        }
                    },
                    Opcode::Lock => {
                        let thread = self.threads.get_mut(id);
                        let lock = self.locks.get_mut(thread.next_lock(program));
                        lock.lock();
                    },
                    Opcode::Unlock => {
                        let thread = self.threads.get_mut(id);
                        let lock = self.locks.get_mut(thread.next_lock(program));
                        lock.unlock();
                    },
                    Opcode::Start => {
                        let thread = self.threads.get_mut(id);
                        let id = thread.next_thread(program);
                        let address = self.registers.get(thread.next_register(program));
                        let other = self.threads.get_mut(id);
                        other.jump(address);
                        other.active = true;
                    },
                    Opcode::Stop => {
                        let thread = self.threads.get_mut(id);
                        let other = thread.next_thread(program);
                        self.threads.get_mut(other).active = false;
                    },
                    Opcode::End => {
                        let thread = self.threads.get_mut(id);
                        thread.active = false;
                    },
                    Opcode::Scan => {
                        let thread = self.threads.get_mut(id);
                        let result = thread.next_register(program);
                        let mut input = String::new();
                        stdin().read_line(&mut input).unwrap();
                        let integer = input.trim().parse::<u64>().unwrap();
                        self.registers.set(result, integer);
                    },
                    Opcode::Print => {
                        let thread = self.threads.get_mut(id);
                        let value = self.registers.get(thread.next_register(program));
                        println!("{}", value);
                    },
                    Opcode::Exit => {
                        break;
                    },
                }
            }
        }
    }
}
