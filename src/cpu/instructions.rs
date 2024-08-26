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
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            _ => None
        }
    }
}
