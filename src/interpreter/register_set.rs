use std::collections::HashMap;

use crate::x86::Register;
use log::trace;

#[derive(Debug)]
pub struct RegisterSet {
    registers: HashMap<Register, i16>,
}

impl RegisterSet {
    pub fn new() -> Self {
        let mut registers = HashMap::new();
        for reg in Register::iter() {
            registers.insert(reg, 0);
        }
        Self { registers }
    }

    pub fn get(&self, reg: Register) -> i16 {
        *self.registers.get(&reg).expect("Unknown register")
    }

    pub fn set(&mut self, reg: Register, value: i16) {
        if let Some(val) = self.registers.get_mut(&reg) {
            trace!("Set {:?}: {:x} (previous: {:x})", reg, value, *val);
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
