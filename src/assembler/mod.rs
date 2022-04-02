pub mod instructions;
pub mod parser;

use std::collections::HashMap;

use parser::Parser;
use instructions::{
    instruction_code,
    instruction_length_size,
    word_opcode,
};

pub struct Assembler {
    code: Box<str>,
    labels: HashMap<Box<str>, usize>,
}

impl Assembler {
    pub fn new(code: Box<str>) -> Self {
        Self {
            code,
            labels: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Box<[u8]> {
        let mut parser = Parser::new(&self.code);
        let mut cursor = 0;
        while let Some(word) = parser.next_word() {
            let opcode = word_opcode(word);
            if opcode.is_none() {
                self.labels.insert(Box::from(word), cursor);
                continue;
            }

            let (length, size) = instruction_length_size(opcode.unwrap());
            for _ in 1 .. length {
                parser.next_word();
            }

            cursor += size;
        }

        let mut parser = Parser::new(&self.code);
        let mut program = Vec::new();
        while let Some(word) = parser.next_word() {
            let opcode = word_opcode(word);
            if opcode.is_none() {
                continue;
            }

            program.extend_from_slice(&instruction_code(opcode.unwrap(), &mut parser, &self.labels));
        }

        program.into_boxed_slice()
    }
}
