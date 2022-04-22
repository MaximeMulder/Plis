use std::io::stdin;
use std::process::exit;
use std::rc::Rc;

use architecture::Opcode;

use crate::lock::{ Locks, LockId };
use crate::memory::Memory;
use crate::program::Program;
use crate::register::Registers;
use crate::thread::{ Threads, Thread, ThreadId };
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
        self.threads.get_mut(ThreadId::from_raw(0)).start();
        loop {
            let threads_ids = self.threads.actives();
            if threads_ids.is_empty() && self.callbacks.is_empty() {
                self.error("No active threads.");
            }

            for thread_id in self.threads.actives().into_iter().copied() {
                let opcode = self.threads.get_mut(thread_id).next_opcode(self.program);
                self.run_instruction(thread_id, opcode);
            }

            for callback in self.callbacks.clone() {
                if callback.0 != self.counter {
                    continue;
                }

                callback.1(self);
            }

            self.callbacks.retain(|callback| callback.0 != self.counter);

            self.counter += 1;
        }
    }

    pub fn run_instruction(&mut self, thread_id: ThreadId, opcode: Opcode) {
        match opcode {
            Opcode::Nop => {},
            Opcode::Const8 => {
                self.constant(thread_id, |thread, program| thread.next_const8(program));
            },
            Opcode::Const16 => {
                self.constant(thread_id, |thread, program| thread.next_const16(program));
            },
            Opcode::Const32 => {
                self.constant(thread_id, |thread, program| thread.next_const32(program));
            },
            Opcode::Const64 => {
                self.constant(thread_id, |thread, program| thread.next_const64(program));
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
                        machine.error_thread(thread_id, "Division by zero.")
                    }

                    a / b
                });
            },
            Opcode::Rem => {
                self.calcul(thread_id, TIME_REM, |_, _, a, b| a % b);
            },
            Opcode::Jump => {
                let thread = self.threads.get_mut(thread_id);

                let address = thread.next_register(self.program);

                let address = self.registers.read(address);

                thread.jump(address);
            },
            Opcode::JumpIf => {
                let thread = self.threads.get_mut(thread_id);

                let address   = thread.next_register(self.program);
                let condition = thread.next_register(self.program);

                let address   = self.registers.read(address);
                let condition = self.registers.read(condition);

                if condition != 0 {
                    thread.jump(address);
                }
            },
            Opcode::Wait => {
                let thread = self.threads.get_mut(thread_id);

                let lock_id = thread.next_lock(self.program);

                if self.locks.get(lock_id).locked() {
                    thread.wait(lock_id);
                }
            },
            Opcode::Lock => {
                let thread = self.threads.get_mut(thread_id);

                let lock_id = thread.next_lock(self.program);

                self.callback(move |machine| {
                    machine.locks.get_mut(lock_id).lock();
                });
            },
            Opcode::Unlock => {
                let thread = self.threads.get_mut(thread_id);

                let lock_id = thread.next_lock(self.program);

                self.callback(move |machine| {
                    machine.unlock(lock_id);
                });
            },
            Opcode::Start => {
                let thread = self.threads.get_mut(thread_id);

                let other   = thread.next_thread(self.program);
                let address = thread.next_register(self.program);

                let address = self.registers.read(address);

                self.callback(move |machine| {
                    let other = machine.threads.get_mut(other);
                    other.jump(address);
                    other.start();
                });
            },
            Opcode::Stop => {
                let thread = self.threads.get_mut(thread_id);

                let other = thread.next_thread(self.program);

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
                let thread = self.threads.get_mut(thread_id);

                let result = thread.next_register(self.program);

                let mut input = String::new();
                stdin().read_line(&mut input).unwrap_or_else(|_| self.error_thread(thread_id, "Cannot read input."));
                let integer = input.trim().parse::<u64>().unwrap_or_else(|_| self.error_thread(thread_id, "Cannot parse input."));
                self.registers.write(result, integer);
            },
            Opcode::Print => {
                let thread = self.threads.get_mut(thread_id);

                let value = thread.next_register(self.program);

                let value = self.registers.read(value);

                println!("{}", value);
            },
            Opcode::Exit => {
                exit(0);
            },
        }
    }

    fn error(&self, message: &str) -> ! {
        println!("ERROR: {}", message);
        exit(0);
    }

    fn error_thread(&self, thread_id: ThreadId, message: &str) -> !{
        let thread = self.threads.get(thread_id);
        eprintln!("ERROR THREAD {} ADRESS {:#X}: {}", thread.id(), thread.cursor(), message);
        exit(0);
    }
}

impl Machine<'_> {
    fn constant(&mut self, thread_id: ThreadId, closure: impl Fn(&mut Thread, &Program) -> u64) {
        let thread = self.threads.get_mut(thread_id);

        let register = thread.next_register(self.program);

        let constant = closure(thread, self.program);

        self.registers.write(register, constant);
    }

    fn load(&mut self, thread_id: ThreadId, closure: fn(&Memory, u64) -> u64) {
        let thread = self.threads.get_mut(thread_id);

        let address     = thread.next_register(self.program);
        let destination = thread.next_register(self.program);
        let lock_id     = thread.next_lock(self.program);

        let address = self.registers.read(address);
        self.locks.get_mut(lock_id).lock();

        self.callback_delay(TIME_LOAD, move |machine| {
            let value = closure(&machine.memory, address);
            machine.registers.write(destination, value);
            machine.unlock(lock_id);
        });
    }

    fn store(&mut self, thread_id: ThreadId, closure: fn(&mut Memory, u64, u64)) {
        let thread = self.threads.get_mut(thread_id);

        let source      = thread.next_register(self.program);
        let destination = thread.next_register(self.program);
        let lock_id     = thread.next_lock(self.program);

        let address = self.registers.read(destination);
        let value   = self.registers.read(source);
        self.locks.get_mut(lock_id).lock();

        self.callback_delay(TIME_STORE, move |machine| {
            closure(&mut machine.memory, address, value);
            machine.unlock(lock_id);
        });
    }

    fn calcul(&mut self, thread_id: ThreadId, delay: usize, closure: fn(&Machine, ThreadId, u64, u64) -> u64) {
        let thread = self.threads.get_mut(thread_id);

        let a       = thread.next_register(self.program);
        let b       = thread.next_register(self.program);
        let result  = thread.next_register(self.program);
        let lock_id = thread.next_lock(self.program);

        let a = self.registers.read(a);
        let b = self.registers.read(b);
        self.locks.get_mut(lock_id).lock();

        self.callback_delay(delay, move |machine| {
            let value = closure(machine, thread_id, a, b);
            machine.registers.write(result, value);
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

    fn unlock(&mut self, lock_id: LockId) {
        self.locks.get_mut(lock_id).unlock();
        for thread in self.threads.actives().into_iter().copied() {
            let thread = self.threads.get_mut(thread);
            if thread.is_waiting(lock_id) {
                thread.start();
            }
        }
    }
}
