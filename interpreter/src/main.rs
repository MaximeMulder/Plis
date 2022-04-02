mod machine;
mod memory;
mod program;
mod thread;
mod register;

use machine::Machine;
use program::Program;

fn main() {
    let program = Program::new(std::fs::read("../test.epismo").unwrap().into_boxed_slice());
    let mut machine = Machine::new();
    machine.run(&program);
}
