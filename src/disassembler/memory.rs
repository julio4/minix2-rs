use super::register::Register;

#[derive(Debug, PartialEq)]
pub struct Memory {
    pub base: Option<Register>,
    pub index: Option<Register>,
    pub disp_low: u8,
    pub disp_high: Option<u8>,
}

impl Memory {
    pub fn new(base: Option<Register>, index: Option<Register>, disp_low: u8) -> Self {
        Memory {
            base,
            index,
            disp_low,
            disp_high: None,
        }
    }

    pub fn new_with_word_disp(base: Option<Register>, index: Option<Register>, disp: u16) -> Self {
        Memory {
            base,
            index,
            disp_low: disp as u8,
            disp_high: Some((disp >> 8) as u8),
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
        // If only disp, convert to EA
        return if base.is_empty() && index.is_empty() && disp != 0 {
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
        assert_eq!(format!("{}", memory), "[bx]");
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
