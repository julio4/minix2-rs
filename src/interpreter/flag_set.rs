use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum Flag {
    Zero,
    Sign,
    Parity,
    Carry,
    Overflow,
    Direction,
    Interrupt,
    Trap,
    Aux,
    PageFault,
}

impl Flag {
    pub fn iter() -> impl Iterator<Item = Flag> {
        [
            Flag::Zero,
            Flag::Sign,
            Flag::Parity,
            Flag::Carry,
            Flag::Overflow,
            Flag::Direction,
            Flag::Interrupt,
            Flag::Trap,
            Flag::Aux,
            Flag::PageFault,
        ]
        .iter()
        .copied()
    }
}

#[derive(Debug)]
pub struct FlagSet {
    flags: HashMap<Flag, bool>,
}

impl FlagSet {
    pub fn new() -> Self {
        let mut flags = HashMap::new();
        for flag in Flag::iter() {
            flags.insert(flag, false);
        }
        Self { flags }
    }

    pub fn get(&self, flag: Flag) -> bool {
        *self.flags.get(&flag).expect("Unknown flag")
    }

    pub fn set(&mut self, flag: Flag, value: bool) {
        if let Some(val) = self.flags.get_mut(&flag) {
            *val = value;
        }
    }

    pub fn clear(&mut self, flag: Flag) {
        self.set(flag, false);
    }
}

impl std::fmt::Display for FlagSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for flag in Flag::iter() {
            write!(f, "{:?}: {}\n", flag, self.get(flag))?;
        }
        Ok(())
    }
}
