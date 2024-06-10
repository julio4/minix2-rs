use super::displacement::Displacement;
use super::Register;

#[derive(Debug, PartialEq)]
pub struct Memory {
    pub base: Option<Register>,
    pub index: Option<Register>,
    pub disp: Option<Displacement>,
}

impl Memory {
    pub fn new(
        base: Option<Register>,
        index: Option<Register>,
        disp: Option<Displacement>,
    ) -> Self {
        Memory { base, index, disp }
    }

    pub fn from_imm(imm: u8) -> Self {
        Memory {
            base: None,
            index: None,
            disp: Some(Displacement::Short(imm as i8)),
        }
    }

    pub fn from_word_imm(imm: u16) -> Self {
        Memory {
            base: None,
            index: None,
            disp: Some(Displacement::Long(imm as i16)),
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

        // If only disp, convert to [imm]
        return if base.is_empty() && index.is_empty() && self.disp.is_some() {
            let value = match self.disp.as_ref().unwrap() {
                Displacement::Short(d) => *d as i16,
                Displacement::Long(d) => *d,
            };
            write!(f, "[{:0>4x}]", value)
        } else {
            write!(f, "[{}{}{}]", base, index, {
                match &self.disp {
                    Some(d) => {
                        if d.is_neg() {
                            format!("{}", d)
                        } else {
                            format!("+{}", d)
                        }
                    }
                    None => "".to_string(),
                }
            })
        };
    }
}

#[cfg(test)]
mod memory_display_test {
    use super::*;

    #[test]
    fn test_memory_display_no_displacement() {
        let memory = Memory {
            base: Some(Register::BX),
            index: None,
            disp: None,
        };
        assert_eq!(format!("{}", memory), "[bx]");
    }

    #[test]
    fn test_memory_display_with_8bits_displacement() {
        let memory = Memory {
            base: Some(Register::BX),
            index: None,
            disp: Some(Displacement::Short(0x5)),
        };
        assert_eq!(format!("{}", memory), "[bx+5]");
    }

    #[test]
    fn test_memory_display_with_16bits_displacement() {
        let memory = Memory {
            base: Some(Register::BX),
            index: None,
            disp: Some(Displacement::Long(0x1000)),
        };
        assert_eq!(format!("{}", memory), "[bx+1000]");
    }

    #[test]
    fn test_memory_display_with_base_index_displacement() {
        let memory = Memory {
            base: Some(Register::BX),
            index: Some(Register::SI),
            disp: Some(Displacement::Short(0x8)),
        };
        assert_eq!(format!("{}", memory), "[bx+si+8]");
    }

    #[test]
    fn test_memory_display_with_displacement_as_ea() {
        let memory = Memory {
            base: None,
            index: None,
            disp: Some(Displacement::Long(0x0010)),
        };
        assert_eq!(format!("{}", memory), "[0010]");
    }

    #[test]
    fn test_memory_display_with_signed_displacement() {
        let memory = Memory {
            base: Some(Register::BX),
            index: Some(Register::SI),
            disp: Some(Displacement::Long((0x89u8 as i8) as i16)),
        };
        assert_eq!(format!("{}", memory), "[bx+si-77]");
    }
}
