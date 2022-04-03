#![feature(decl_macro)]
#![feature(let_else)]

mod assembler;
mod instructions;
mod operand;
mod parser;

use std::env::args;
use std::path::Path;

use assembler::Assembler;

fn main() {
    let arguments = args().collect::<Box<[_]>>();
    if arguments.len() != 3 {
        panic!();
    }

    let input = get_input_path(&arguments[1]);
    let output = get_output_path(&arguments[2]);
    let code = std::fs::read_to_string(input).unwrap().into_boxed_str();
    let mut parser = Assembler::new(code);
    let program = parser.parse();
    std::fs::write(output, program).unwrap();
}

fn get_input_path(argument: &str) -> &Path {
    let path = Path::new(argument);
    let Some(extension) = path.extension() else {
        panic!();
    };

    if extension != "epism" {
        panic!();
    }

    path
}

fn get_output_path(argument: &str) -> &Path {
    let path = Path::new(argument);
    let Some(extension) = path.extension() else {
        panic!();
    };

    if extension != "epismo" {
        panic!();
    }

    path
}
