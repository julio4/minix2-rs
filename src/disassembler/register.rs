#[derive(Debug, PartialEq)]
pub enum Register {
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
}

impl Register {
    pub fn from(reg: u8, w: bool) -> Self {
        if reg > 0b111 {
            panic!("Invalid register number: {}", reg);
        }
        let key = (reg & 0b111) | ((w as u8) << 3);
        match key {
            // 8-bit registers
            0b0000 => Register::AL,
            0b0001 => Register::CL,
            0b0010 => Register::DL,
            0b0011 => Register::BL,
            0b0100 => Register::AH,
            0b0101 => Register::CH,
            0b0110 => Register::DH,
            0b0111 => Register::BH,
            // 16-bit registers
            0b1000 => Register::AX,
            0b1001 => Register::CX,
            0b1010 => Register::DX,
            0b1011 => Register::BX,
            0b1100 => Register::SP,
            0b1101 => Register::BP,
            0b1110 => Register::SI,
            0b1111 => Register::DI,
            _ => unreachable!(),
        }
    }
}
