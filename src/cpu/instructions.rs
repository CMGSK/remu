pub enum Instruction {
    HALT,
    NOP,
    ADD(ArithmeticTarget),
    INC(IncDecTarget),
    CALL(JumpTest),
    RET(JumpTest),
    JP(JumpTest),
    RLC(PrefixTarget),
    LD(LoadType),
}

pub enum PrefixTarget {
    B,
}

pub enum IncDecTarget {
    BC
}

pub enum ArithmeticTarget {
    A, 
    B, 
    C, 
    D, 
    E, 
    H, 
    L,
}

pub enum LoadByteTarget {
    A, 
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

pub enum LoadByteSource {
    A, 
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource)
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

impl Instruction {
    pub fn from_byte(byte: u8, prefix_instr: bool) -> Option<Instruction> {
        if prefix_instr {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_unprefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            _ => None
        }
    }

    fn from_byte_unprefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            _ => None
        }
    }
}
