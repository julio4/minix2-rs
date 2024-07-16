use std::collections::HashMap;

use crate::x86::Register;

#[derive(Debug)]
pub struct RegisterSet {
    registers: HashMap<Register, u16>,
}

impl RegisterSet {
    pub fn new() -> Self {
        let mut registers = HashMap::new();
        for reg in Register::iter_16() {
            registers.insert(reg, 0);
        }
        Self { registers }
    }

    pub fn get(&self, reg: Register) -> u16 {
        if reg.is_word_register() {
            return *self.registers.get(&reg).unwrap();
        }
        let word_value = *self
            .registers
            .get(&reg.to_word_register())
            .expect("Unknown register");
        return if reg.is_low_byte() {
            word_value & 0xFF
        } else {
            (word_value >> 8) & 0xFF
        };
    }

    pub fn set(&mut self, reg: Register, value: u16) {
        let word_reg = reg.to_word_register();
        let reg_val = self.get(word_reg);
        let mut value = value;
        if reg.is_low_byte() {
            value = (value & 0xFF) | (reg_val & 0xFF00);
        }
        if reg.is_high_byte() {
            value = (value & 0xFF00) | (reg_val & 0xFF);
        }

        if let Some(val) = self.registers.get_mut(&word_reg) {
            *val = value;
        }
    }
}

impl std::fmt::Display for RegisterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for reg in Register::iter() {
            write!(f, "{:?}: {:x}\n", reg, self.get(reg))?;
        }
        Ok(())
    }
}
