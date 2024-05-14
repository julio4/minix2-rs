#[derive(Debug, Clone, PartialEq)]
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

    pub fn get_base(rm: u8) -> Option<Register> {
        match rm {
            0b000 | 0b001 | 0b111 => Some(Register::BX),
            0b010 | 0b011 | 0b110 => Some(Register::BP),
            0b100 => Some(Register::SI),
            0b101 => Some(Register::DI),
            _ => None,
        }
    }

    pub fn get_index(rm: u8) -> Option<Register> {
        match rm {
            0b000 | 0b010 => Some(Register::SI),
            0b001 | 0b011 => Some(Register::DI),
            _ => None,
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = match self {
            Register::AX => "ax",
            Register::CX => "cx",
            Register::DX => "dx",
            Register::BX => "bx",
            Register::SP => "sp",
            Register::BP => "bp",
            Register::SI => "si",
            Register::DI => "di",
            Register::AL => "al",
            Register::CL => "cl",
            Register::DL => "dl",
            Register::BL => "bl",
            Register::AH => "ah",
            Register::CH => "ch",
            Register::DH => "dh",
            Register::BH => "bh",
        };
        write!(f, "{}", reg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_from() {
        assert_eq!(Register::from(0b000, false), Register::AL);
        assert_eq!(Register::from(0b001, false), Register::CL);
        assert_eq!(Register::from(0b010, false), Register::DL);
        assert_eq!(Register::from(0b011, false), Register::BL);
        assert_eq!(Register::from(0b100, false), Register::AH);
        assert_eq!(Register::from(0b101, false), Register::CH);
        assert_eq!(Register::from(0b110, false), Register::DH);
        assert_eq!(Register::from(0b111, false), Register::BH);
        assert_eq!(Register::from(0b000, true), Register::AX);
        assert_eq!(Register::from(0b001, true), Register::CX);
        assert_eq!(Register::from(0b010, true), Register::DX);
        assert_eq!(Register::from(0b011, true), Register::BX);
        assert_eq!(Register::from(0b100, true), Register::SP);
        assert_eq!(Register::from(0b101, true), Register::BP);
        assert_eq!(Register::from(0b110, true), Register::SI);
        assert_eq!(Register::from(0b111, true), Register::DI);
    }

    #[test]
    #[should_panic(expected = "Invalid register number: 16")]
    fn test_register_from_invalid() {
        Register::from(0b10000, false);
    }

    #[test]
    fn test_get_base() {
        assert_eq!(Register::get_base(0b000), Some(Register::BX));
        assert_eq!(Register::get_base(0b001), Some(Register::BX));
        assert_eq!(Register::get_base(0b111), Some(Register::BX));
        assert_eq!(Register::get_base(0b010), Some(Register::BP));
        assert_eq!(Register::get_base(0b011), Some(Register::BP));
        assert_eq!(Register::get_base(0b110), Some(Register::BP));
        assert_eq!(Register::get_base(0b100), Some(Register::SI));
        assert_eq!(Register::get_base(0b101), Some(Register::DI));
        assert_eq!(Register::get_base(0b1111), None);
    }

    #[test]
    fn test_get_index() {
        assert_eq!(Register::get_index(0b000), Some(Register::SI));
        assert_eq!(Register::get_index(0b010), Some(Register::SI));
        assert_eq!(Register::get_index(0b001), Some(Register::DI));
        assert_eq!(Register::get_index(0b011), Some(Register::DI));
        assert_eq!(Register::get_index(0b100), None);
        assert_eq!(Register::get_index(0b101), None);
        assert_eq!(Register::get_index(0b1111), None);
    }
}
