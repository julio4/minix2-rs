use std::fmt::Display;

use super::{memory::Memory, register::Register};

#[derive(Debug, PartialEq)]
pub enum Operand {
    Register(Register),
    Immediate(u8),
    LongImmediate(u16),
    Memory(Memory),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Register(r) => write!(f, "{}", r),
            Operand::Immediate(i) => write!(f, "{:02x}", i),
            Operand::LongImmediate(i) => write!(f, "{:04x}", i),
            Operand::Memory(mem) => write!(f, "{}", mem),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum IR {
    Mov { dest: Operand, src: Operand },
    // Push { src: Operand },
    // Pop { dest: Operand },
    Int { int_type: u8 },
}

impl Display for IR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IR::Mov { dest, src } => write!(f, "mov {}, {}", dest, src),
            IR::Int { int_type } => {
                if *int_type == 3 {
                    write!(f, "int")
                } else {
                    write!(f, "int {:02x}", int_type)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    ir: IR,
    raw: Vec<u8>,
}

impl Instruction {
    pub fn new(ir: IR, raw: Vec<u8>) -> Self {
        Instruction { ir, raw }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // raw      ir
        // 0xbb00   mov bx, 0x0000
        // cd20     int 20
        write!(f, "{:02x?}\t{}", self.raw, self.ir)
            }
}
