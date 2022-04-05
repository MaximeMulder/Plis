use architecture::Opcode;
use std::collections::HashMap;

use crate::operand::Operand;
use crate::parser::Parser;
use crate::instructions::{ word_opcode, opcode_operands };

pub struct Assembler {
    code: Box<str>,
}

impl Assembler {
    pub fn new(code: Box<str>) -> Self {
        Self {
            code,
        }
    }

    pub fn parse(&mut self) -> Box<[u8]> {
        let labels = self.parse_labels();
        self.parse_instructions(labels)
    }

    fn parse_labels(&self) -> HashMap<Box<str>, usize> {
        let mut parser = Parser::new(&self.code);
        let mut labels = HashMap::new();
        let mut address = 0;
        while let Some(word) = parser.next_word() {
            let Some(opcode) = word_opcode(word) else {
                let label = Box::from(word);
                if labels.contains_key(&label) {
                    parser.error("Label already exists.")
                }

                if !parser.next_colon() {
                    parser.error("Missing colon.");
                }

                labels.insert(label, address);
                continue;
            };

            address += 1;
            let operands = opcode_operands(opcode);
            parser.with_operands(operands, |operand, _| {
                address += operand.size();
            });
        }

        labels
    }

    fn parse_instructions(&self, labels: HashMap<Box<str>, usize>) -> Box<[u8]> {
        let mut parser = Parser::new(&self.code);
        let mut program = Vec::new();
        while let Some(word) = parser.next_word() {
            let Some(opcode) = word_opcode(word) else {
                parser.next_colon();
                continue;
            };

            program.push(Opcode::to_raw(opcode));
            let operands = opcode_operands(opcode);
            for operand in operands {
                parser.next_comma();
                let word = parser.next_word().unwrap();
                if let Err(error) = operand.parse(word, &mut program, &labels) {
                    parser.error(&error);
                }
            }
        }

        program.into_boxed_slice()
    }
}

impl Parser<'_> {
    fn with_operands(&mut self, operands: &[Operand], mut closure: impl FnMut(Operand, &str)) {
        let mut iterator = operands.iter().copied();
        let Some(operand) = iterator.next() else {
            return;
        };

        self.with_operand(operand, |operand, word| closure(operand, word));
        for operand in iterator {
            if !self.next_comma() {
                self.error("Missing comma.");
            }

            self.with_operand(operand, |operand, word| closure(operand, word));
        }
    }

    fn with_operand(&mut self, operand: Operand, mut closure: impl FnMut(Operand, &str)) {
        let Some(word) = self.next_word() else {
            self.error("Missing operand.");
        };

        closure(operand, word);
    }
}
