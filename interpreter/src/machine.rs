mod error;
mod lock;
mod memory;
mod register;
mod thread;

use std::io::stdin;
use std::process::exit;
use std::rc::Rc;

use architecture::Opcode;

use lock::Locks;
use memory::Memory;
use register::Registers;
use thread::{ Threads, ThreadId };

use crate::program::Program;
use crate::time::*;

pub struct Machine<'a> {
    program: &'a Program,
    threads: Threads,
    registers: Registers,
    locks: Locks,
    memory: Memory,
    callbacks: Vec<(usize, Rc<dyn Fn(&mut Machine)>)>,
    counter: usize,
}

impl<'a> Machine<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            threads: Threads::new(),
            registers: Registers::new(),
            locks: Locks::new(),
            memory: Memory::new(),
            callbacks: Vec::new(),
            counter: 0,
        }
    }

    pub fn run(&mut self) {
        self.threads.get_mut(ThreadId::from_raw(0).unwrap()).start();
        loop {
            let actives = self.threads.get_actives();
            if actives.is_empty() && self.callbacks.is_empty() {
                self.error_pause();
            }

            for thread in self.threads.get_actives().into_iter().copied() {
                let opcode = self.next_opcode(thread);
                self.run_instruction(thread, opcode);
            }

            for callback in self.callbacks.clone() {
                if callback.0 != self.counter {
                    continue;
                }

                callback.1(self);
            }

            self.callbacks.retain(|callback| callback.0 != self.counter);

            self.registers.reset();
            self.counter += 1;
        }
    }

    pub fn run_instruction(&mut self, thread_id: ThreadId, opcode: Opcode) {
        match opcode {
            Opcode::Nop => {},
            Opcode::Move => {
                let source      = self.next_register(thread_id);
                let destination = self.next_register(thread_id);

                let value = self.register_read(source);
                self.register_write(destination, value);
            },
            Opcode::Const8 => {
                self.constant(thread_id, |machine, thread| machine.next_const8(thread));
            },
            Opcode::Const16 => {
                self.constant(thread_id, |machine, thread| machine.next_const16(thread));
            },
            Opcode::Const32 => {
                self.constant(thread_id, |machine, thread| machine.next_const32(thread));
            },
            Opcode::Const64 => {
                self.constant(thread_id, |machine, thread| machine.next_const64(thread));
            },
            Opcode::Load8 => {
                self.load(thread_id, |memory, address| memory.get_8(address));
            },
            Opcode::Load16 => {
                self.load(thread_id, |memory, address| memory.get_16(address));
            },
            Opcode::Load32 => {
                self.load(thread_id, |memory, address| memory.get_32(address));
            },
            Opcode::Load64 => {
                self.load(thread_id, |memory, address| memory.get_64(address));
            },
            Opcode::Store8 => {
                self.store(thread_id, |memory, address, value| memory.set_8(address, value as u8));
            },
            Opcode::Store16 => {
                self.store(thread_id, |memory, address, value| memory.set_16(address, value as u16));
            },
            Opcode::Store32 => {
                self.store(thread_id, |memory, address, value| memory.set_32(address, value as u32));
            },
            Opcode::Store64 => {
                self.store(thread_id, |memory, address, value| memory.set_64(address, value));
            },
            Opcode::And => {
                self.calcul(thread_id, TIME_AND, |_, _, a, b| a & b);
            },
            Opcode::Or => {
                self.calcul(thread_id, TIME_OR,  |_, _, a, b| a | b);
            },
            Opcode::Xor => {
                self.calcul(thread_id, TIME_XOR, |_, _, a, b| a ^ b);
            },
            Opcode::ShiftL => {
                self.calcul(thread_id, TIME_SHL, |_, _, a, b| a << b);
            },
            Opcode::ShiftR => {
                self.calcul(thread_id, TIME_SHR, |_, _, a, b| a >> b);
            },
            Opcode::Add => {
                self.calcul(thread_id, TIME_ADD, |_, _, a, b| a + b);
            },
            Opcode::Sub => {
                self.calcul(thread_id, TIME_SUB, |_, _, a, b| a - b);
            },
            Opcode::Mul => {
                self.calcul(thread_id, TIME_MUL, |_, _, a, b| a * b);
            },
            Opcode::Div => {
                self.calcul(thread_id, TIME_DIV, |machine, thread_id, a, b| {
                    if b == 0 {
                        machine.error_division_by_zero(thread_id);
                    }

                    a / b
                });
            },
            Opcode::Rem => {
                self.calcul(thread_id, TIME_REM, |_, _, a, b| a % b);
            },
            Opcode::Jump => {
                let address = self.next_register(thread_id);

                let address = self.register_read(address);

                let thread = self.threads.get_mut(thread_id);
                thread.jump(address);
            },
            Opcode::JumpIf => {
                let address   = self.next_register(thread_id);
                let condition = self.next_register(thread_id);

                let address   = self.register_read(address);
                let condition = self.register_read(condition);

                if condition != 0 {
                    let thread = self.threads.get_mut(thread_id);
                    thread.jump(address);
                }
            },
            Opcode::Wait => {
                let lock_id = self.next_lock(thread_id);

                if self.locked(lock_id) {
                    let thread = self.threads.get_mut(thread_id);
                    thread.wait(lock_id);
                }
            },
            Opcode::Lock => {
                let lock_id = self.next_lock(thread_id);

                self.callback(move |machine| {
                    machine.lock(lock_id);
                });
            },
            Opcode::Unlock => {
                let lock_id = self.next_lock(thread_id);

                self.callback(move |machine| {
                    machine.unlock(lock_id);
                });
            },
            Opcode::Start => {
                let other   = self.next_thread(thread_id);
                let address = self.next_register(thread_id);

                let address = self.register_read(address);

                self.callback(move |machine| {
                    let other = machine.threads.get_mut(other);
                    other.jump(address);
                    other.start();
                });
            },
            Opcode::Stop => {
                let other   = self.next_thread(thread_id);

                self.callback(move |machine| {
                    let other = machine.threads.get_mut(other);
                    other.stop();
                });
            },
            Opcode::End => {
                let thread = self.threads.get_mut(thread_id);

                thread.stop();
            },
            Opcode::Scan => {
                let result = self.next_register(thread_id);

                let mut input = String::new();
                stdin().read_line(&mut input).unwrap_or_else(|_| self.error_input_read(thread_id));
                let integer = input.trim().parse::<u64>().unwrap_or_else(|_| self.error_input_parse(thread_id));
                self.register_write(result, integer);
            },
            Opcode::Print => {
                let value = self.next_register(thread_id);

                let value = self.register_read(value);

                println!("{}", value);
            },
            Opcode::Exit => {
                exit(0);
            },
        }
    }
}

impl Machine<'_> {
    fn constant(&mut self, thread: ThreadId, closure: impl Fn(&mut Machine, ThreadId) -> u64) {
        let register = self.next_register(thread);

        let constant = closure(self, thread);

        self.register_write(register, constant);
    }

    fn load(&mut self, thread_id: ThreadId, closure: fn(&Memory, u64) -> u64) {
        let address     = self.next_register(thread_id);
        let destination = self.next_register(thread_id);
        let lock_id     = self.next_lock(thread_id);

        let address = self.register_read(address);
        self.lock(lock_id);

        self.callback_delay(TIME_LOAD, move |machine| {
            let value = closure(&machine.memory, address);
            machine.register_write(destination, value);
            machine.unlock(lock_id);
        });
    }

    fn store(&mut self, thread_id: ThreadId, closure: fn(&mut Memory, u64, u64)) {
        let source      = self.next_register(thread_id);
        let destination = self.next_register(thread_id);
        let lock_id     = self.next_lock(thread_id);

        let address = self.register_read(destination);
        let value   = self.register_read(source);
        self.lock(lock_id);

        self.callback_delay(TIME_STORE, move |machine| {
            closure(&mut machine.memory, address, value);
            machine.unlock(lock_id);
        });
    }

    fn calcul(&mut self, thread_id: ThreadId, delay: usize, closure: fn(&Machine, ThreadId, u64, u64) -> u64) {
        let a       = self.next_register(thread_id);
        let b       = self.next_register(thread_id);
        let result  = self.next_register(thread_id);
        let lock_id = self.next_lock(thread_id);

        let a = self.register_read(a);
        let b = self.register_read(b);
        self.lock(lock_id);

        self.callback_delay(delay, move |machine| {
            let value = closure(machine, thread_id, a, b);
            machine.register_write(result, value);
            machine.unlock(lock_id);
        });
    }
}

impl Machine<'_> {
    fn callback(&mut self, callback: impl Fn(&mut Machine) + 'static) {
        self.callbacks.push((self.counter, Rc::new(callback)));
    }

    fn callback_delay(&mut self, delay: usize, callback: impl Fn(&mut Machine) + 'static) {
        self.callbacks.push((self.counter + delay, Rc::new(callback)));
    }
}
