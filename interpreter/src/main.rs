#![allow(dead_code)]
#![feature(bool_to_option)]
#![feature(let_else)]

mod machine;
mod program;
mod time;

use std::fs::read;
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
    let program = Program::new(read(input).unwrap().into_boxed_slice());
    let mut machine = Machine::new(&program);
    machine.run();
}

fn get_input_path(argument: &str) -> &Path {
    let path = Path::new(argument);
    let Some(extension) = path.extension() else {
        panic!();
    };

    if extension != "pliso" {
        panic!();
    }

    path
}
