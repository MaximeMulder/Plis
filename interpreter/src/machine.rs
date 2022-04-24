mod error;
mod instructions;
mod lock;
mod memory;
mod register;
mod thread;

use std::io::stdin;
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

            for thread in self.threads.iter_mut() {
                thread.profile_update();
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
                self.instruction_const(thread_id, |machine, thread_id| machine.next_const8(thread_id));
            },
            Opcode::Const16 => {
                self.instruction_const(thread_id, |machine, thread_id| machine.next_const16(thread_id));
            },
            Opcode::Const32 => {
                self.instruction_const(thread_id, |machine, thread_id| machine.next_const32(thread_id));
            },
            Opcode::Const64 => {
                self.instruction_const(thread_id, |machine, thread_id| machine.next_const64(thread_id));
            },
            Opcode::Load8 => {
                self.instruction_load(thread_id, |machine, thread_id, address| machine.load8(thread_id, address) as u64);
            },
            Opcode::Load16 => {
                self.instruction_load(thread_id, |machine, thread_id, address| machine.load16(thread_id, address) as u64);
            },
            Opcode::Load32 => {
                self.instruction_load(thread_id, |machine, thread_id, address| machine.load32(thread_id, address) as u64);
            },
            Opcode::Load64 => {
                self.instruction_load(thread_id, |machine, thread_id, address| machine.load64(thread_id, address) as u64);
            },
            Opcode::Store8 => {
                self.instruction_store(thread_id, |machine, thread_id, address, value| machine.store8(thread_id, address, value as u8));
            },
            Opcode::Store16 => {
                self.instruction_store(thread_id, |machine, thread_id, address, value| machine.store16(thread_id, address, value as u16));
            },
            Opcode::Store32 => {
                self.instruction_store(thread_id, |machine, thread_id, address, value| machine.store32(thread_id, address, value as u32));
            },
            Opcode::Store64 => {
                self.instruction_store(thread_id, |machine, thread_id, address, value| machine.store64(thread_id, address, value as u64));
            },
            Opcode::And => {
                self.instruction_calcul(thread_id, TIME_AND, |_, _, a, b| a & b);
            },
            Opcode::Or => {
                self.instruction_calcul(thread_id, TIME_OR,  |_, _, a, b| a | b);
            },
            Opcode::Xor => {
                self.instruction_calcul(thread_id, TIME_XOR, |_, _, a, b| a ^ b);
            },
            Opcode::ShiftL => {
                self.instruction_calcul(thread_id, TIME_SHL, |_, _, a, b| a << b);
            },
            Opcode::ShiftR => {
                self.instruction_calcul(thread_id, TIME_SHR, |_, _, a, b| a >> b);
            },
            Opcode::Add => {
                self.instruction_calcul(thread_id, TIME_ADD, |_, _, a, b| a + b);
            },
            Opcode::Sub => {
                self.instruction_calcul(thread_id, TIME_SUB, |_, _, a, b| a - b);
            },
            Opcode::Mul => {
                self.instruction_calcul(thread_id, TIME_MUL, |_, _, a, b| a * b);
            },
            Opcode::Div => {
                self.instruction_calcul(thread_id, TIME_DIV, |machine, thread_id, a, b| {
                    if b == 0 {
                        machine.error_division_by_zero(thread_id);
                    }

                    a / b
                });
            },
            Opcode::Rem => {
                self.instruction_calcul(thread_id, TIME_REM, |machine, thread_id, a, b| {
                    if b == 0 {
                        machine.error_division_by_zero(thread_id);
                    }

                    a % b
                });
            },
            Opcode::Eq => {
                self.instruction_calcul(thread_id, TIME_EQ, |_, _, a, b| if a == b { 0 } else { 1 });
            },
            Opcode::Lt => {
                self.instruction_calcul(thread_id, TIME_LT, |_, _, a, b| if a < b { 0 } else { 1 });
            },
            Opcode::Gt => {
                self.instruction_calcul(thread_id, TIME_GT, |_, _, a, b| if a > b { 0 } else { 1 });
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

                if condition == 0 {
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
            Opcode::Halt => {
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
            Opcode::ProfileReset => {
                self.instruction_profile_reset();
            },
            Opcode::ProfileDump => {
                self.instruction_profile_dump();
            },
            Opcode::End => {
                self.instruction_end();
            },
        }
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
