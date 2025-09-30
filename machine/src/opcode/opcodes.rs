use std::convert::TryFrom;

#[derive(Debug)]
pub enum Opcodes {
    PUSH = 0x0001,
    POP = 0x0002,
    MOV = 0x0003,
    ADD = 0x0004,
    SUB = 0x0005,
    DROP = 0x0006,
    DUP = 0x0007,
    SWP = 0x0008,
    INC = 0x0009,
    DEC = 0x000a,
    JMP = 0x000b,
    JZ = 0x000c,
    JNZ = 0x000d,
    JG = 0x000e,
    JGE = 0x000f,
    JL = 0x0010,
    JLE = 0x0011,
    AND = 0x0012,
    OR = 0x0013,
    NOT = 0x0014,
    XOR = 0x0015,
    SHR = 0x0016,
    SHL = 0x0017,
    ROL = 0x0018,
    ROR = 0x0019,
    HLT = 0x001a,
    YLD = 0x001b,
}

impl From<Opcodes> for u16 {
    fn from(op: Opcodes) -> Self {
        op as u16
    }
}

impl TryFrom<u16> for Opcodes {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0001 => Ok(Opcodes::PUSH),
            0x0002 => Ok(Opcodes::POP),
            0x0003 => Ok(Opcodes::MOV),
            0x0004 => Ok(Opcodes::ADD),
            0x0005 => Ok(Opcodes::SUB),
            0x0006 => Ok(Opcodes::DROP),
            0x0007 => Ok(Opcodes::DUP),
            0x0008 => Ok(Opcodes::SWP),
            0x0009 => Ok(Opcodes::INC),
            0x000a => Ok(Opcodes::DEC),
            0x000b => Ok(Opcodes::JMP),
            0x000c => Ok(Opcodes::JZ),
            0x000d => Ok(Opcodes::JNZ),
            0x000e => Ok(Opcodes::JG),
            0x000f => Ok(Opcodes::JGE),
            0x0010 => Ok(Opcodes::JL),
            0x0011 => Ok(Opcodes::JLE),
            0x0012 => Ok(Opcodes::AND),
            0x0013 => Ok(Opcodes::OR),
            0x0014 => Ok(Opcodes::NOT),
            0x0015 => Ok(Opcodes::XOR),
            0x0016 => Ok(Opcodes::SHR),
            0x0017 => Ok(Opcodes::SHL),
            0x0018 => Ok(Opcodes::ROL),
            0x0019 => Ok(Opcodes::ROR),
            0x001a => Ok(Opcodes::HLT),
            0x001b => Ok(Opcodes::YLD),
            _ => Err(()),
        }
    }
}