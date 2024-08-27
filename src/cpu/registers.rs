pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: Flags,
    pub g: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    pub fn set_bc(&mut self, val: u16) {
        self.b = ((val & 0xFF00) >> 8) as u8;
        self.c = (val &0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    pub fn set_de(&mut self, val: u16) {
        self.d = ((val & 0xFF00) >> 8) as u8;
        self.e = (val &0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
    pub fn set_hl(&mut self, val: u16) {
        self.h = ((val & 0xFF00) >> 8) as u8;
        self.l = (val &0xFF) as u8;
    }

    //af is not accessible because f stands for flag register
}

// From the f register
pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

//This addresses the position of the byte in decimal system
const ZERO_FLAG: u8 = 7;
const SUBTRACT_FLAG: u8 = 6;
const HALF_CARRY_FLAG: u8 = 5;
const CARRY_FLAG: u8 = 4;

// From trait allows us to convert Flag into u8 an viceversa
impl From<Flags> for u8 {
    fn from(flags: Flags) -> u8 {
        (flags.zero as u8) << ZERO_FLAG |
        (flags.subtract as u8) << SUBTRACT_FLAG |
        (flags.half_carry as u8) << HALF_CARRY_FLAG |
        (flags.carry as u8) << CARRY_FLAG
    }
}

impl From<u8> for Flags {
    fn from(byte: u8) -> Self {
        // Place the significant byte into the position 0 then AND it with 00000001 to get the value of the byte
        // then cast to bool
        let zero = ((byte >> ZERO_FLAG) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG) & 0b1) != 0;

        Flags{
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}