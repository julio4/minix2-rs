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
