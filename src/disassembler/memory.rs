use super::register::Register;

#[derive(Debug, PartialEq)]
pub struct Memory {
    pub base: Option<Register>,
    pub index: Option<Register>,
    pub disp_low: u8,
    pub disp_high: Option<u8>,
}

impl Memory {
    pub fn from_modrm(mod_: u8, rm: u8, disp_bytes: &[u8], w: bool) -> Memory {
        match mod_ {
            0b11 => Memory {
                base: Some(Register::from(rm, w)),
                index: None,
                disp_low: 0,
                disp_high: None,
            },
            0b00 => {
                // Special case *: EA = disp-high;disp-low
                if rm == 0b110 {
                    Memory {
                        base: None,
                        index: None,
                        disp_low: disp_bytes[0],
                        disp_high: Some(disp_bytes[1]),
                    }
                } else {
                    Memory {
                        base: Self::get_base_register(rm),
                        index: Self::get_index_register(rm),
                        disp_low: 0,
                        disp_high: None,
                    }
                }
            }
            0b01 => Memory {
                base: Self::get_base_register(rm),
                index: Self::get_index_register(rm),
                disp_low: disp_bytes[0],
                disp_high: None,
            },
            0b10 => Memory {
                base: Self::get_base_register(rm),
                index: Self::get_index_register(rm),
                disp_low: disp_bytes[0],
                disp_high: Some(disp_bytes[1]),
            },
            _ => unreachable!(),
        }
    }

    fn get_base_register(rm: u8) -> Option<Register> {
        match rm {
            0b000 | 0b001 | 0b111 => Some(Register::BX),
            0b010 | 0b011 | 0b110 => Some(Register::BP),
            0b100 => Some(Register::SI),
            0b101 => Some(Register::DI),
            _ => None,
        }
    }

    fn get_index_register(rm: u8) -> Option<Register> {
        match rm {
            0b000 | 0b010 => Some(Register::SI),
            0b001 | 0b011 => Some(Register::DI),
            _ => None,
        }
    }
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = match &self.base {
            Some(b) => format!("{}", b),
            None => "".to_string(),
        };
        let index = match &self.index {
            Some(i) => format!("+{}", i),
            None => "".to_string(),
        };
        let disp = match self.disp_high {
            Some(d) => (d as u16) << 8 | (self.disp_low as u16),
            None => self.disp_low as u16,
        };
        // If only base, don't print []
        return if !base.is_empty() && index.is_empty() && disp == 0 {
            // TODO: should we explicitly convert to reg?
            write!(f, "{}", base)
        }
        // If only disp, convert to EA
        else if base.is_empty() && index.is_empty() && disp != 0 {
            write!(f, "0x{:04x}", disp)
        } else {
            write!(f, "[{}{}{}]", base, index, {
                if disp != 0 {
                    format!("+{}", disp)
                } else {
                    "".to_string()
                }
            })
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_display_no_displacement() {
        let memory = Memory {
            disp_low: 0,
            base: Some(Register::BX),
            index: None,
            disp_high: None,
        };
        assert_eq!(format!("{}", memory), "bx");
    }

    #[test]
    fn test_memory_display_with_8bits_displacement() {
        let memory = Memory {
            disp_low: 5,
            base: Some(Register::BX),
            index: None,
            disp_high: None,
        };
        assert_eq!(format!("{}", memory), "[bx+5]");
    }

    #[test]
    fn test_memory_display_with_16bits_displacement() {
        let memory = Memory {
            disp_low: 0x00,
            base: Some(Register::BX),
            index: None,
            disp_high: Some(0x10),
        };
        assert_eq!(format!("{}", memory), "[bx+4096]");
    }

    #[test]
    fn test_memory_display_with_base_index_displacement() {
        let memory = Memory {
            disp_low: 8,
            base: Some(Register::BX),
            index: Some(Register::SI),
            disp_high: None,
        };
        assert_eq!(format!("{}", memory), "[bx+si+8]");
    }

    #[test]
    fn test_memory_display_with_displacement_as_ea() {
        let memory = Memory {
            disp_low: 0,
            base: None,
            index: None,
            disp_high: Some(0x10),
        };
        assert_eq!(format!("{}", memory), "0x1000");
    }
}
