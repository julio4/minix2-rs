use crate::x86::Executable;

use super::register_set::RegisterSet;

struct VM {
    // cpu
    pub ip: u16,
    // memory
    // todo
    // registers
    pub registers: RegisterSet,
    pub flags: u16,
}

impl Default for VM {
    fn default() -> Self {
        VM {
            flags: 0,
            ip: 0,
            registers: RegisterSet::new(),
        }
    }
}

impl VM {
    // todo
}

pub fn interpret(executable: Executable) {
    let mut vm = VM::default();
    // todo
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::x86::IR;

    #[test]
    fn test_interpret() {
        let exe = Executable::from(vec![IR::Undefined]);
        interpret(exe);
    }
}
