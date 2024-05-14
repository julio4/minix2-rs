use super::register::Register;

#[derive(Debug, PartialEq)]
pub struct Memory {
    pub base: Option<Register>,
    pub index: Option<Register>,
    pub disp_low: u8,
    pub disp_high: Option<u8>,
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = match &self.base {
            Some(b) => format!("{}", b),
            None => "".to_string(),
        };
        let index = match &self.index {
            Some(i) => format!("{}{}", self.base.as_ref().map_or("", |_| "+"), i),
            None => "".to_string(),
        };
        let disp = match self.disp_high {
            Some(d) => (d as u16) << 8 | (self.disp_low as u16),
            None => self.disp_low as u16,
        };
        let disp = if disp == 0 {
            "".to_string()
        } else {
            format!("{:+}", disp)
        };
        write!(f, "[{}{}{}]", base, index, disp)
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
    fn test_memory_display_with_index_only() {
        let memory = Memory {
            disp_low: 0,
            base: None,
            index: Some(Register::SI),
            disp_high: None,
        };
        assert_eq!(format!("{}", memory), "[si]");
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
}
