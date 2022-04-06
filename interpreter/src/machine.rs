use std::io::stdin;

use architecture::Opcode;

use crate::lock::Locks;
use crate::memory::Memory;
use crate::program::Program;
use crate::register::Registers;
use crate::thread::{ Threads, ThreadId, Thread };

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
                        self.constant(id, program, |thread, program| thread.next_const8(program));
                    },
                    Opcode::Const16 => {
                        self.constant(id, program, |thread, program| thread.next_const16(program));
                    },
                    Opcode::Const32 => {
                        self.constant(id, program, |thread, program| thread.next_const32(program));
                    },
                    Opcode::Const64 => {
                        self.constant(id, program, |thread, program| thread.next_const64(program));
                    },
                    Opcode::Load8 => {
                        self.load(id, program, |memory, address| memory.get_8(address));
                    },
                    Opcode::Load16 => {
                        self.load(id, program, |memory, address| memory.get_16(address));
                    },
                    Opcode::Load32 => {
                        self.load(id, program, |memory, address| memory.get_32(address));
                    },
                    Opcode::Load64 => {
                        self.load(id, program, |memory, address| memory.get_64(address));
                    },
                    Opcode::Store8 => {
                        self.store(id, program, |memory, address, value| memory.set_8(address, value as u8));
                    },
                    Opcode::Store16 => {
                        self.store(id, program, |memory, address, value| memory.set_16(address, value as u16));
                    },
                    Opcode::Store32 => {
                        self.store(id, program, |memory, address, value| memory.set_32(address, value as u32));
                    },
                    Opcode::Store64 => {
                        self.store(id, program, |memory, address, value| memory.set_64(address, value));
                    },
                    Opcode::And => {
                        self.calcul(id, program, |a, b| a & b);
                    },
                    Opcode::Or => {
                        self.calcul(id, program, |a, b| a | b);
                    },
                    Opcode::Xor => {
                        self.calcul(id, program, |a, b| a ^ b);
                    },
                    Opcode::ShiftL => {
                        self.calcul(id, program, |a, b| a << b);
                    },
                    Opcode::ShiftR => {
                        self.calcul(id, program, |a, b| a >> b);
                    },
                    Opcode::Add => {
                        self.calcul(id, program, |a, b| a + b);
                    },
                    Opcode::Sub => {
                        self.calcul(id, program, |a, b| a - b);
                    },
                    Opcode::Mul => {
                        self.calcul(id, program, |a, b| a * b);
                    },
                    Opcode::Div => {
                        self.calcul(id, program, |a, b| a / b);
                    },
                    Opcode::Rem => {
                        self.calcul(id, program, |a, b| a % b);
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

impl Machine {
    fn constant(&mut self, id: ThreadId, program: &Program, closure: impl Fn(&mut Thread, &Program) -> u64) {
        let thread = self.threads.get_mut(id);
        let register = thread.next_register(program);
        let constant = closure(thread, program);
        self.registers.set(register, constant);
    }

    fn load(&mut self, id: ThreadId, program: &Program, closure: impl Fn(&Memory, u64) -> u64) {
        let thread = self.threads.get_mut(id);
        let source = thread.next_register(program);
        let destination = thread.next_register(program);
        let _lock = thread.next_lock(program);
        let value = closure(&self.memory, self.registers.get(source));
        self.registers.set(destination, value);
    }

    fn store(&mut self, id: ThreadId, program: &Program, closure: impl Fn(&mut Memory, u64, u64)) {
        let thread = self.threads.get_mut(id);
        let source = thread.next_register(program);
        let destination = thread.next_register(program);
        let _lock = thread.next_lock(program);
        let address = self.registers.get(destination);
        let value = self.registers.get(source);
        closure(&mut self.memory, address, value);
    }

    fn calcul(&mut self, id: ThreadId, program: &Program, closure: impl Fn(u64, u64) -> u64) {
        let thread = self.threads.get_mut(id);
        let a = self.registers.get(thread.next_register(program));
        let b = self.registers.get(thread.next_register(program));
        let result = thread.next_register(program);
        let _lock = thread.next_lock(program);
        self.registers.set(result, closure(a, b));
    }
}
