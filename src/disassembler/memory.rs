use super::register::Register;

#[derive(Debug, PartialEq)]
pub struct Memory {
    pub base: Option<Register>,
    pub index: Option<Register>,
    pub disp_low: u8,
    pub disp_high: Option<u8>,
}

