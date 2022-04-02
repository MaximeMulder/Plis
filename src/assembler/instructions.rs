use std::collections::HashMap;

use crate::opcode::Opcode;
use crate::assembler::parser::Parser;

pub fn word_opcode(word: &str) -> Option<Opcode> {
    Some(match word {
        "const8"  => Opcode::Const8,
        "const16" => Opcode::Const16,
        "const32" => Opcode::Const32,
        "const64" => Opcode::Const64,
        "load8"   => Opcode::Load8,
        "load16"  => Opcode::Load16,
        "load32"  => Opcode::Load32,
        "load64"  => Opcode::Load64,
        "store8"  => Opcode::Store8,
        "store16" => Opcode::Store16,
        "store32" => Opcode::Store32,
        "store64" => Opcode::Store64,
        "and"     => Opcode::And,
        "or"      => Opcode::Or,
        "xor"     => Opcode::Xor,
        "shl"     => Opcode::ShiftL,
        "shr"     => Opcode::ShiftR,
        "add"     => Opcode::Add,
        "sub"     => Opcode::Sub,
        "mul"     => Opcode::Mul,
        "div"     => Opcode::Div,
        "rem"     => Opcode::Rem,
        "jump"    => Opcode::Jump,
        "jumpif"  => Opcode::JumpIf,
        "wait"    => Opcode::Wait,
        "lock"    => Opcode::Lock,
        "unlock"  => Opcode::Unlock,
        "start"   => Opcode::Start,
        "stop"    => Opcode::Stop,
        "end"     => Opcode::End,
        "scan"    => Opcode::Scan,
        "print"   => Opcode::Print,
        "exit"    => Opcode::Exit,
        _ => return None,
    })
}

pub fn instruction_length_size(opcode: Opcode) -> (usize, usize) {
    match opcode {
        Opcode::Nop     => (1, 1),
        Opcode::Const8  => (3, 3),
        Opcode::Const16 => (3, 4),
        Opcode::Const32 => (3, 6),
        Opcode::Const64 => (3, 10),
        Opcode::Load8   => (4, 4),
        Opcode::Load16  => (4, 4),
        Opcode::Load32  => (4, 4),
        Opcode::Load64  => (4, 4),
        Opcode::Store8  => (4, 4),
        Opcode::Store16 => (4, 4),
        Opcode::Store32 => (4, 4),
        Opcode::Store64 => (4, 4),
        Opcode::And     => (5, 5),
        Opcode::Or      => (5, 5),
        Opcode::Xor     => (5, 5),
        Opcode::ShiftL  => (5, 5),
        Opcode::ShiftR  => (5, 5),
        Opcode::Add     => (5, 5),
        Opcode::Sub     => (5, 5),
        Opcode::Mul     => (5, 5),
        Opcode::Div     => (5, 5),
        Opcode::Rem     => (5, 5),
        Opcode::Jump    => (2, 2),
        Opcode::JumpIf  => (3, 3),
        Opcode::Wait    => (2, 2),
        Opcode::Lock    => (2, 2),
        Opcode::Unlock  => (2, 2),
        Opcode::Start   => (3, 3),
        Opcode::Stop    => (2, 2),
        Opcode::End     => (1, 1),
        Opcode::Scan    => (2, 2),
        Opcode::Print   => (2, 2),
        Opcode::Exit    => (1, 1),
    }
}

pub fn instruction_code(opcode: Opcode, parser: &mut Parser, labels: &HashMap<Box<str>, usize>) -> Box<[u8]> {
    let mut program = Vec::new();
    program.push(Opcode::to_raw(opcode));
    match opcode {
        Opcode::Nop     => {},
        Opcode::Const8  => {
            program.push(parser.expect_register());
            program.extend_from_slice(&parser.expect_const8(labels));
        },
        Opcode::Const16 => {
            program.push(parser.expect_register());
            program.extend_from_slice(&parser.expect_const16(labels));
        },
        Opcode::Const32 => {
            program.push(parser.expect_register());
            program.extend_from_slice(&parser.expect_const32(labels));
        },
        Opcode::Const64 => {
            program.push(parser.expect_register());
            program.extend_from_slice(&parser.expect_const64(labels));
        },
        Opcode::Load8   => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Load16  => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Load32  => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Load64  => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Store8  => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Store16 => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Store32 => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Store64 => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::And     => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Or      => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Xor     => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::ShiftL  => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::ShiftR  => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Add     => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Sub     => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Mul     => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Div     => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Rem     => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_register());
            program.push(parser.expect_lock());
        },
        Opcode::Jump    => {
            program.push(parser.expect_register());
        },
        Opcode::JumpIf  => {
            program.push(parser.expect_register());
            program.push(parser.expect_register());
        },
        Opcode::Wait    => {
            program.push(parser.expect_lock());
        },
        Opcode::Lock    => {
            program.push(parser.expect_lock());
        },
        Opcode::Unlock  => {
            program.push(parser.expect_lock());
        },
        Opcode::Start   => {
            program.push(parser.expect_thread());
        },
        Opcode::Stop    => {
            program.push(parser.expect_thread());
        },
        Opcode::End     => {},
        Opcode::Scan    => {
            program.push(parser.expect_register());
        },
        Opcode::Print   => {
            program.push(parser.expect_register());
        },
        Opcode::Exit    => {},
    }

    program.into_boxed_slice()
}
