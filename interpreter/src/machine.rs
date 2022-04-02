use architecture::Opcode;

use crate::memory::Memory;
use crate::program::Program;
use crate::register::Registers;
use crate::thread::Threads;

pub struct Machine {
    threads: Threads,
    registers: Registers,
    memory: Memory,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            threads: Threads::new(),
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self, program: &Program) {
        self.threads.threads[0].active = true;
        loop {
            for thread in self.threads.threads.iter_mut() {
                if !thread.active {
                    continue;
                }

                let opcode = thread.next_opcode(program);
                match opcode {
                    Opcode::Nop => {},
                    Opcode::Const8 => {
                        let register = thread.next_register(program);
                        let constant = thread.next_const8(program);
                        self.registers.set(register, constant);
                    },
                    Opcode::Const16 => {
                        let register = thread.next_register(program);
                        let constant = thread.next_const16(program);
                        self.registers.set(register, constant);
                    },
                    Opcode::Const32 => {
                        let register = thread.next_register(program);
                        let constant = thread.next_const32(program);
                        self.registers.set(register, constant);
                    },
                    Opcode::Const64 => {
                        let register = thread.next_register(program);
                        let constant = thread.next_const64(program);
                        self.registers.set(register, constant);
                    },
                    Opcode::Load8 => {
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let address = self.registers.get(source);
                        let value = self.memory.get_8(address);
                        self.registers.set(destination, value);
                    },
                    Opcode::Load16 => {
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let address = self.registers.get(source);
                        let value = self.memory.get_16(address);
                        self.registers.set(destination, value);
                    },
                    Opcode::Load32 => {
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let address = self.registers.get(source);
                        let value = self.memory.get_32(address);
                        self.registers.set(destination, value);
                    },
                    Opcode::Load64 => {
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let value = self.memory.get_64(self.registers.get(source));
                        self.registers.set(destination, value);
                    },
                    Opcode::Store8 => {
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let address = self.registers.get(destination);
                        let value = self.registers.get(source) as u8;
                        self.memory.set_8(address, value);
                    },
                    Opcode::Store16 => {
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let address = self.registers.get(destination);
                        let value = self.registers.get(source) as u16;
                        self.memory.set_16(address, value);
                    },
                    Opcode::Store32 => {
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let address = self.registers.get(destination);
                        let value = self.registers.get(source) as u32;
                        self.memory.set_32(address, value);
                    },
                    Opcode::Store64 => {
                        let source = thread.next_register(program);
                        let destination = thread.next_register(program);
                        let address = self.registers.get(destination);
                        let value = self.registers.get(source);
                        self.memory.set_64(address, value);
                    },
                    Opcode::And => {
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        self.registers.set(result, a & b);
                    },
                    Opcode::Or => {
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        self.registers.set(result, a | b);
                    },
                    Opcode::Xor => {
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        self.registers.set(result, a ^ b);
                    },
                    Opcode::ShiftL => {
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        self.registers.set(result, a << b);
                    },
                    Opcode::ShiftR => {
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        self.registers.set(result, a >> b);
                    },
                    Opcode::Add => {
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        self.registers.set(result, a + b);
                    },
                    Opcode::Sub => {
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        self.registers.set(result, a - b);
                    },
                    Opcode::Mul => {
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        self.registers.set(result, a * b);
                    },
                    Opcode::Div => {
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        self.registers.set(result, a / b);
                    },
                    Opcode::Rem => {
                        let a = self.registers.get(thread.next_register(program));
                        let b = self.registers.get(thread.next_register(program));
                        let result = thread.next_register(program);
                        self.registers.set(result, a % b);
                    },
                    Opcode::Jump => {
                        let address = self.registers.get(thread.next_register(program));
                        thread.jump(address);
                    },
                    Opcode::JumpIf => {
                        let address = self.registers.get(thread.next_register(program));
                        let condition = self.registers.get(thread.next_register(program));
                        if condition != 0 {
                            thread.jump(address);
                        }
                    },
                    Opcode::Wait => {
                        // TODO
                    },
                    Opcode::Lock => {
                        // TODO
                    },
                    Opcode::Unlock => {
                        // TODO
                    },
                    Opcode::Start => {
                        // TODO
                    },
                    Opcode::Stop => {
                        // TODO
                    },
                    Opcode::End => {
                        // TODO
                    },
                    Opcode::Scan => {
                        // TODO
                    },
                    Opcode::Print => {
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
