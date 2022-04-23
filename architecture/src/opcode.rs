#[derive(Clone, Copy)]
pub enum Opcode {
    Nop,
    Move,
    Const8,
    Const16,
    Const32,
    Const64,
    Load8,
    Load16,
    Load32,
    Load64,
    Store8,
    Store16,
    Store32,
    Store64,
    And,
    Or,
    Xor,
    ShiftL,
    ShiftR,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Gt,
    Jump,
    JumpIf,
    Wait,
    Lock,
    Unlock,
    Start,
    Stop,
    End,
    Scan,
    Print,
    Exit,
}

impl Opcode {
    pub fn from_raw(raw: u8) -> Option<Opcode> {
        Some(match raw {
            0x00 => Opcode::Nop,
            0x01 => Opcode::Move,
            0x02 => Opcode::Const8,
            0x03 => Opcode::Const16,
            0x04 => Opcode::Const32,
            0x05 => Opcode::Const64,
            0x06 => Opcode::Load8,
            0x07 => Opcode::Load16,
            0x08 => Opcode::Load32,
            0x09 => Opcode::Load64,
            0x0A => Opcode::Store8,
            0x0B => Opcode::Store16,
            0x0C => Opcode::Store32,
            0x0D => Opcode::Store64,
            0x0E => Opcode::And,
            0x0F => Opcode::Or,
            0x10 => Opcode::Xor,
            0x11 => Opcode::ShiftL,
            0x12 => Opcode::ShiftR,
            0x13 => Opcode::Add,
            0x14 => Opcode::Sub,
            0x15 => Opcode::Mul,
            0x16 => Opcode::Div,
            0x17 => Opcode::Rem,
            0xF0 => Opcode::Eq,
            0xF1 => Opcode::Gt,
            0x18 => Opcode::Jump,
            0x19 => Opcode::JumpIf,
            0x1A => Opcode::Wait,
            0x1B => Opcode::Lock,
            0x1C => Opcode::Unlock,
            0x1D => Opcode::Start,
            0x1E => Opcode::Stop,
            0x1F => Opcode::End,
            0x20 => Opcode::Scan,
            0x21 => Opcode::Print,
            0x22 => Opcode::Exit,
            _ => return None,
        })
    }

    pub fn to_raw(opcode: Opcode) -> u8 {
        match opcode {
            Opcode::Nop     => 0x00,
            Opcode::Move    => 0x01,
            Opcode::Const8  => 0x02,
            Opcode::Const16 => 0x03,
            Opcode::Const32 => 0x04,
            Opcode::Const64 => 0x05,
            Opcode::Load8   => 0x06,
            Opcode::Load16  => 0x07,
            Opcode::Load32  => 0x08,
            Opcode::Load64  => 0x09,
            Opcode::Store8  => 0x0A,
            Opcode::Store16 => 0x0B,
            Opcode::Store32 => 0x0C,
            Opcode::Store64 => 0x0D,
            Opcode::And     => 0x0E,
            Opcode::Or      => 0x0F,
            Opcode::Xor     => 0x10,
            Opcode::ShiftL  => 0x11,
            Opcode::ShiftR  => 0x12,
            Opcode::Add     => 0x13,
            Opcode::Sub     => 0x14,
            Opcode::Mul     => 0x15,
            Opcode::Div     => 0x16,
            Opcode::Rem     => 0x17,
            Opcode::Eq      => 0xF0,
            Opcode::Gt      => 0xF1,
            Opcode::Jump    => 0x18,
            Opcode::JumpIf  => 0x19,
            Opcode::Wait    => 0x1A,
            Opcode::Lock    => 0x1B,
            Opcode::Unlock  => 0x1C,
            Opcode::Start   => 0x1D,
            Opcode::Stop    => 0x1E,
            Opcode::End     => 0x1F,
            Opcode::Scan    => 0x20,
            Opcode::Print   => 0x21,
            Opcode::Exit    => 0x22,
        }
    }
}
