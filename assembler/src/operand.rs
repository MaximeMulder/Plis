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
            Operand::Const8   => program.extend_from_slice(&parse_const8(word, labels)),
            Operand::Const16  => program.extend_from_slice(&parse_const16(word, labels)),
            Operand::Const32  => program.extend_from_slice(&parse_const32(word, labels)),
            Operand::Const64  => program.extend_from_slice(&parse_const64(word, labels)),
            Operand::Register => program.push(parse_register(word)),
            Operand::Lock     => program.push(parse_lock(word)),
            Operand::Thread   => program.push(parse_thread(word)),
        }
    }
}

fn check_integer(word: &str) -> bool {
    word.chars().all(|character| character.is_numeric())
}

fn parse_label(word: &str, labels: &HashMap<Box<str>, usize>) -> usize {
    if let Some(address) = labels.get(word).copied() {
        return address;
    }

    panic!("Error label.");
}

macro parse_const($type:ty, $word:expr, $labels:expr) {{
    let constant = if check_integer($word) {
        <$type>::from_str_radix($word, 10).unwrap_or_else(|_| panic!("Constant error."))
    } else {
        <$type>::try_from(parse_label($word, $labels)).unwrap_or_else(|_| panic!("Label error."))
    };

    constant.to_be_bytes()
}}

fn parse_const8(word: &str, labels: &HashMap<Box<str>, usize>) -> [u8; 1] {
    parse_const!(u8, word, labels)
}

fn parse_const16(word: &str, labels: &HashMap<Box<str>, usize>) -> [u8; 2] {
    parse_const!(u16, word, labels)
}

fn parse_const32(word: &str, labels: &HashMap<Box<str>, usize>) -> [u8; 4] {
    parse_const!(u32, word, labels)
}

fn parse_const64(word: &str, labels: &HashMap<Box<str>, usize>) -> [u8; 8] {
    parse_const!(u64, word, labels)
}

fn parse_register(word: &str) -> u8 {
    let (prefix, index) = word.split_at(1);
    if prefix != "r" {
        panic!("Error register prefix");
    }

    let Ok(register) = u8::from_str_radix(index, 10) else {
        panic!("Error register index");
    };

    register.to_be()
}

fn parse_lock(word: &str) -> u8 {
    let (prefix, index) = word.split_at(1);
    if prefix != "l" {
        panic!("Error lock prefix");
    }

    let Ok(lock) = u8::from_str_radix(index, 10) else {
        panic!("Error lock index");
    };

    lock.to_be()
}

fn parse_thread(word: &str) -> u8 {
    let (prefix, index) = word.split_at(1);
    if prefix != "t" {
        panic!("Error thread prefix");
    }

    let Ok(thread) = u8::from_str_radix(index, 10) else {
        panic!("Error thread index");
    };

    thread.to_be()
}
