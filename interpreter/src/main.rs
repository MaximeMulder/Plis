#![feature(let_else)]

mod lock;
mod machine;
mod memory;
mod program;
mod thread;
mod register;

use std::env::args;
use std::path::Path;

use machine::Machine;
use program::Program;

fn main() {
    let arguments = args().collect::<Box<[_]>>();
    if arguments.len() != 2 {
        panic!();
    }

    let input = get_input_path(&arguments[1]);
    let program = Program::new(std::fs::read(input).unwrap().into_boxed_slice());
    let mut machine = Machine::new();
    machine.run(&program);
}

fn get_input_path(argument: &str) -> &Path {
    let path = Path::new(argument);
    let Some(extension) = path.extension() else {
        panic!();
    };

    if extension != "epismo" {
        panic!();
    }

    path
}