use super::register::Register;

#[derive(Debug, PartialEq)]
pub enum Operand {
    Register(Register),
    Immediate(u8),
    LongImmediate(u16),
    Memory(u16),
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mov { dest: Operand, src: Operand },
    Push { src: Operand },
    Pop { dest: Operand },
}
