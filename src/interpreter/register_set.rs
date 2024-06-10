use std::collections::HashMap;

use crate::x86::Register;

#[derive(Debug)]
pub struct RegisterSet {
    registers: HashMap<Register, i32>,
}

impl RegisterSet {
    pub fn new() -> Self {
        let mut registers = HashMap::new();
        for reg in Register::iter() {
            registers.insert(reg, 0);
        }
        Self { registers }
    }

    pub fn get(&self, reg: Register) -> Option<&i32> {
        self.registers.get(&reg)
    }

    pub fn set(&mut self, reg: Register, value: i32) {
        if let Some(val) = self.registers.get_mut(&reg) {
            *val = value;
        }
    }
}
