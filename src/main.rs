#![feature(let_else)]

#![allow(dead_code)]


use assembler::Assembler;

use machine::Machine;
use program::Program;

mod assembler;
mod machine;
mod memory;
mod opcode;
mod program;
mod thread;
mod register;

fn main() {
    let program = Program::new(std::fs::read("test.epismo").unwrap().into_boxed_slice());
    let mut machine = Machine::new();
    machine.run(&program);
    /* let mut parser = Assembler::new(std::fs::read_to_string("test.epism").unwrap().into_boxed_str());
    std::fs::write("test.epismo", parser.parse()).unwrap(); */
}
