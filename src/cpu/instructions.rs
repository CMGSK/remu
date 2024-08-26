pub enum ArithmeticTarget {
    A, 
    B, 
    C, 
    D, 
    E, 
    H, 
    L,
}

pub enum Instruction {
    ADD(ArithmeticTarget),
    INC(ArithmeticTarget),
    JP(JumpTest),
    RLC(PrefixTarget),
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
