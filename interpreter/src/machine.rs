use std::io::stdin;

use architecture::Opcode;

use crate::lock::Locks;
use crate::memory::Memory;
use crate::program::Program;
use crate::register::Registers;
use crate::thread::{ Threads, ThreadId, Thread };

pub struct Machine<'a> {
    program: &'a Program,
    threads: Threads,
    registers: Registers,
    locks: Locks,
    memory: Memory,
}

impl<'a> Machine<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            threads: Threads::new(),
            registers: Registers::new(),
            locks: Locks::new(),
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self) {
        self.threads.get_mut(ThreadId::from_raw(0)).active = true;
        loop {
            for thread in self.threads.iterate() {
                {
                    let thread = self.threads.get(thread);
                    if !thread.active {
                        continue;
                    }
                }

                let opcode = self.threads.get_mut(thread).next_opcode(self.program);
                match opcode {
                    Opcode::Nop => {},
                    Opcode::Const8 => {
                        self.constant(thread, |thread, program| thread.next_const8(program));
                    },
                    Opcode::Const16 => {
                        self.constant(thread, |thread, program| thread.next_const16(program));
                    },
                    Opcode::Const32 => {
                        self.constant(thread, |thread, program| thread.next_const32(program));
                    },
                    Opcode::Const64 => {
                        self.constant(thread, |thread, program| thread.next_const64(program));
                    },
                    Opcode::Load8 => {
                        self.load(thread, |memory, address| memory.get_8(address));
                    },
                    Opcode::Load16 => {
                        self.load(thread, |memory, address| memory.get_16(address));
                    },
                    Opcode::Load32 => {
                        self.load(thread, |memory, address| memory.get_32(address));
                    },
                    Opcode::Load64 => {
                        self.load(thread, |memory, address| memory.get_64(address));
                    },
                    Opcode::Store8 => {
                        self.store(thread, |memory, address, value| memory.set_8(address, value as u8));
                    },
                    Opcode::Store16 => {
                        self.store(thread, |memory, address, value| memory.set_16(address, value as u16));
                    },
                    Opcode::Store32 => {
                        self.store(thread, |memory, address, value| memory.set_32(address, value as u32));
                    },
                    Opcode::Store64 => {
                        self.store(thread, |memory, address, value| memory.set_64(address, value));
                    },
                    Opcode::And => {
                        self.calcul(thread, |a, b| a & b);
                    },
                    Opcode::Or => {
                        self.calcul(thread, |a, b| a | b);
                    },
                    Opcode::Xor => {
                        self.calcul(thread, |a, b| a ^ b);
                    },
                    Opcode::ShiftL => {
                        self.calcul(thread, |a, b| a << b);
                    },
                    Opcode::ShiftR => {
                        self.calcul(thread, |a, b| a >> b);
                    },
                    Opcode::Add => {
                        self.calcul(thread, |a, b| a + b);
                    },
                    Opcode::Sub => {
                        self.calcul(thread, |a, b| a - b);
                    },
                    Opcode::Mul => {
                        self.calcul(thread, |a, b| a * b);
                    },
                    Opcode::Div => {
                        self.calcul(thread, |a, b| a / b);
                    },
                    Opcode::Rem => {
                        self.calcul(thread, |a, b| a % b);
                    },
                    Opcode::Jump => {
                        let thread = self.threads.get_mut(thread);

                        let address = thread.next_register(self.program);

                        let address = self.registers.read(address);

                        thread.jump(address);
                    },
                    Opcode::JumpIf => {
                        let thread = self.threads.get_mut(thread);

                        let address   = thread.next_register(self.program);
                        let condition = thread.next_register(self.program);

                        let address   = self.registers.read(address);
                        let condition = self.registers.read(condition);

                        if condition != 0 {
                            thread.jump(address);
                        }
                    },
                    Opcode::Wait => {
                        let thread = self.threads.get_mut(thread);

                        let lock = thread.next_lock(self.program);

                        let lock = self.locks.get(lock);

                        if lock.locked() {
                            thread.wait();
                        }
                    },
                    Opcode::Lock => {
                        let thread = self.threads.get_mut(thread);

                        let lock = thread.next_lock(self.program);

                        let lock = self.locks.get_mut(lock);

                        lock.lock();
                    },
                    Opcode::Unlock => {
                        let thread = self.threads.get_mut(thread);

                        let lock = thread.next_lock(self.program);

                        let lock = self.locks.get_mut(lock);

                        lock.unlock();
                    },
                    Opcode::Start => {
                        let thread = self.threads.get_mut(thread);

                        let other   = thread.next_thread(self.program);
                        let address = thread.next_register(self.program);

                        let other   = self.threads.get_mut(other);
                        let address = self.registers.read(address);

                        other.jump(address);
                        other.active = true;
                    },
                    Opcode::Stop => {
                        let thread = self.threads.get_mut(thread);

                        let other = thread.next_thread(self.program);

                        self.threads.get_mut(other).active = false;
                    },
                    Opcode::End => {
                        let thread = self.threads.get_mut(thread);

                        thread.active = false;
                    },
                    Opcode::Scan => {
                        let thread = self.threads.get_mut(thread);

                        let result = thread.next_register(self.program);

                        let mut input = String::new();
                        stdin().read_line(&mut input).unwrap();
                        let integer = input.trim().parse::<u64>().unwrap();
                        self.registers.write(result, integer);
                    },
                    Opcode::Print => {
                        let thread = self.threads.get_mut(thread);

                        let value = thread.next_register(self.program);

                        let value = self.registers.read(value);

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

impl Machine<'_> {
    fn constant(&mut self, thread: ThreadId, closure: impl Fn(&mut Thread, &Program) -> u64) {
        let thread = self.threads.get_mut(thread);

        let register = thread.next_register(self.program);

        let constant = closure(thread, self.program);

        self.registers.write(register, constant);
    }

    fn load(&mut self, thread: ThreadId, closure: impl Fn(&Memory, u64) -> u64) {
        let thread = self.threads.get_mut(thread);

        let address     = thread.next_register(self.program);
        let destination = thread.next_register(self.program);
        let _lock       = thread.next_lock(self.program);

        let address = self.registers.read(address);
        let value   = closure(&self.memory, address);

        self.registers.write(destination, value);
    }

    fn store(&mut self, thread: ThreadId, closure: impl Fn(&mut Memory, u64, u64)) {
        let thread = self.threads.get_mut(thread);

        let source      = thread.next_register(self.program);
        let destination = thread.next_register(self.program);
        let _lock       = thread.next_lock(self.program);

        let address = self.registers.read(destination);
        let value   = self.registers.read(source);

        closure(&mut self.memory, address, value);
    }

    fn calcul(&mut self, thread: ThreadId, closure: impl Fn(u64, u64) -> u64) {
        let thread = self.threads.get_mut(thread);

        let a      = thread.next_register(self.program);
        let b      = thread.next_register(self.program);
        let result = thread.next_register(self.program);
        let _lock  = thread.next_lock(self.program);

        let a = self.registers.read(a);
        let b = self.registers.read(b);

        self.registers.write(result, closure(a, b));
    }
}
