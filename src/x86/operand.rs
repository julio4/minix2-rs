use super::displacement::Displacement;
use super::Address;
use super::Register;

/// An operand represent the possible values that can be used as an argument for instructions,
/// such as registers, memory addresses, immediates, ...
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operand {
    Register(Register),
    Immediate(u8),
    LongImmediate(u16),
    SignExtendedImmediate(i8),
    MemoryAddress(Address),
    Displacement(Displacement),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Register(r) => write!(f, "{}", r),
            Operand::Immediate(i) => write!(f, "{:x}", i),
            Operand::LongImmediate(i) => write!(f, "{:04x}", i),
            Operand::SignExtendedImmediate(i) => {
                if i.is_negative() {
                    write!(f, "-{:x}", i.abs())
                } else {
                    write!(f, "{:x}", i)
                }
            }
            Operand::MemoryAddress(mem) => write!(f, "{}", mem),
            Operand::Displacement(d) => match d {
                Displacement::Short(d) => write!(f, "{:02x}", d),
                Displacement::Long(d) => write!(f, "{:04x}", d),
            },
        }
    }
}
