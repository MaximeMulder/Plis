use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum Operand {
    Const8,
    Const16,
    Const32,
    Const64,
    Register,
    Lock,
    Thread,
}

impl Operand {
    pub fn size(self) -> usize {
        match self {
            Operand::Const8   => 1,
            Operand::Const16  => 2,
            Operand::Const32  => 4,
            Operand::Const64  => 8,
            Operand::Register => 1,
            Operand::Lock     => 1,
            Operand::Thread   => 1,
        }
    }

    pub fn parse(self, word: &str, program: &mut Vec<u8>, labels: &HashMap<Box<str>, usize>) {
        match self {
            Operand::Const8 => {
                let constant = word.parse::<u8>().unwrap_or_else(|_| {
                    labels.get(word).unwrap().clone().try_into().unwrap()
                });

                program.extend_from_slice(&constant.to_be_bytes());
            },
            Operand::Const16 => {
                let constant = word.parse::<u16>().unwrap_or_else(|_| {
                    labels.get(word).unwrap().clone().try_into().unwrap()
                });

                program.extend_from_slice(&constant.to_be_bytes());
            },
            Operand::Const32 => {
                let constant = word.parse::<u32>().unwrap_or_else(|_| {
                    labels.get(word).unwrap().clone().try_into().unwrap()
                });

                program.extend_from_slice(&constant.to_be_bytes());
            },
            Operand::Const64 => {
                let constant = word.parse::<u64>().unwrap_or_else(|_| {
                    labels.get(word).unwrap().clone().try_into().unwrap()
                });

                program.extend_from_slice(&constant.to_be_bytes());
            },
            Operand::Register => {
                let constant = word.parse::<u8>().unwrap();
                program.push(constant.to_be());
            },
            Operand::Lock => {
                let constant = word.parse::<u8>().unwrap();
                program.push(constant.to_be());
            },
            Operand::Thread => {
                let constant = word.parse::<u8>().unwrap();
                program.push(constant.to_be());
            },
        }
    }
}
