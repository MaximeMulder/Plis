pub mod instructions;
pub mod operand;
pub mod parser;

use std::collections::HashMap;

use parser::Parser;
use instructions::{ word_opcode, opcode_operands };

use crate::opcode::Opcode;

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

            cursor += 1;
            let operands = opcode_operands(opcode.unwrap());
            for operand in operands {
                parser.next_word();
                cursor += operand.size();
            }
        }

        let mut parser = Parser::new(&self.code);
        let mut program = Vec::new();
        while let Some(word) = parser.next_word() {
            let opcode = word_opcode(word);
            if opcode.is_none() {
                continue;
            }

            program.push(Opcode::to_raw(opcode.unwrap()));
            let operands = opcode_operands(opcode.unwrap());
            for operand in operands {
                operand.parse(parser.next_word().unwrap(), &mut program, &self.labels);
            }
        }

        program.into_boxed_slice()
    }
}
