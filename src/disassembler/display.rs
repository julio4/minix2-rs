use crate::x86::{Displacement, Memory, Operand, Register, IR};

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = match self {
            Register::AX => "ax",
            Register::CX => "cx",
            Register::DX => "dx",
            Register::BX => "bx",
            Register::SP => "sp",
            Register::BP => "bp",
            Register::SI => "si",
            Register::DI => "di",
            Register::AL => "al",
            Register::CL => "cl",
            Register::DL => "dl",
            Register::BL => "bl",
            Register::AH => "ah",
            Register::CH => "ch",
            Register::DH => "dh",
            Register::BH => "bh",
        };
        write!(f, "{}", reg)
    }
}

impl std::fmt::Display for IR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IR::Mov { dest, src, byte } => write!(
                f,
                "mov {}{}, {}",
                // show byte for word registers
                match dest {
                    Operand::Register(reg) => {
                        if reg.is_word_register() && *byte {
                            "byte "
                        } else {
                            ""
                        }
                    }
                    _ =>
                        if *byte {
                            "byte "
                        } else {
                            ""
                        },
                },
                dest,
                src
            ),
            IR::Push { src } => write!(f, "push {}", src),
            IR::Pop { dest } => write!(f, "pop {}", dest),
            IR::Xchg { dest, src } => write!(f, "xchg {}, {}", dest, src),
            IR::In { dest, src } => write!(f, "in {}, {}", dest, src),
            IR::Out { dest, src } => write!(f, "out {}, {}", dest, src),
            IR::Xlat => write!(f, "xlat"),
            IR::Lea { dest, src } => write!(f, "lea {}, {}", dest, src),
            IR::Lds { dest, src } => write!(f, "lds {}, {}", dest, src),
            IR::Les { dest, src } => write!(f, "les {}, {}", dest, src),
            IR::Lahf => write!(f, "lahf"),
            IR::Sahf => write!(f, "sahf"),
            IR::Pushf => write!(f, "pushf"),
            IR::Popf => write!(f, "popf"),
            IR::Add { dest, src } => write!(f, "add {}, {}", dest, src),
            IR::Adc { dest, src } => write!(f, "adc {}, {}", dest, src),
            IR::Inc { dest } => write!(f, "inc {}", dest),
            IR::Aaa => write!(f, "aaa"),
            IR::Baa => write!(f, "baa"),
            IR::Sub { dest, src } => write!(f, "sub {}, {}", dest, src),
            IR::Ssb { dest, src } => write!(f, "sbb {}, {}", dest, src),
            IR::Dec { dest } => write!(f, "dec {}", dest),
            IR::Neg { dest } => write!(f, "neg {}", dest),
            IR::Cmp { dest, src, byte } => write!(
                f,
                "cmp {}{}, {}",
                match dest {
                    Operand::Register(reg) => {
                        if reg.is_word_register() && *byte {
                            "byte "
                        } else {
                            ""
                        }
                    }
                    _ =>
                        if *byte {
                            "byte "
                        } else {
                            ""
                        },
                },
                dest,
                src
            ),
            IR::Aas => write!(f, "aas"),
            IR::Das => write!(f, "das"),
            IR::Mul { dest } => write!(f, "mul {}", dest),
            IR::Imul { dest } => write!(f, "imul {}", dest),
            IR::Aam => write!(f, "aam"),
            IR::Div { dest } => write!(f, "div {}", dest),
            IR::Idiv { dest } => write!(f, "idiv {}", dest),
            IR::Aad => write!(f, "aad"),
            IR::Cbw => write!(f, "cbw"),
            IR::Cwd => write!(f, "cwd"),
            IR::Not { dest } => write!(f, "not {}", dest),
            IR::Shl { dest, src } => write!(f, "shl {}, {}", dest, src),
            IR::Shr { dest, src } => write!(f, "shr {}, {}", dest, src),
            IR::Sar { dest, src } => write!(f, "sar {}, {}", dest, src),
            IR::Rol { dest, src } => write!(f, "rol {}, {}", dest, src),
            IR::Ror { dest, src } => write!(f, "ror {}, {}", dest, src),
            IR::Rcl { dest, src } => write!(f, "rcl {}, {}", dest, src),
            IR::Rcr { dest, src } => write!(f, "rcr {}, {}", dest, src),
            IR::And { dest, src } => write!(f, "and {}, {}", dest, src),
            IR::Test { dest, src, byte } => write!(
                f,
                "test {}{}, {}",
                // show byte for word registers
                match dest {
                    Operand::Register(reg) => {
                        if reg.is_word_register() && *byte {
                            "byte "
                        } else {
                            ""
                        }
                    }
                    _ =>
                        if *byte {
                            "byte "
                        } else {
                            ""
                        },
                },
                dest,
                src
            ),
            IR::Or { dest, src } => write!(f, "or {}, {}", dest, src),
            IR::Xor { dest, src } => write!(f, "xor {}, {}", dest, src),
            IR::Rep { z: _, string_ir } => write!(f, "rep {}", string_ir),
            IR::Movs { word } => write!(f, "movs{}", if *word { "w" } else { "b" }),
            IR::Cmps { word } => write!(f, "cmps{}", if *word { "w" } else { "b" }),
            IR::Scas { word } => write!(f, "scas{}", if *word { "w" } else { "b" }),
            IR::Lods { word } => write!(f, "lods{}", if *word { "w" } else { "b" }),
            IR::Stos { word } => write!(f, "stos{}", if *word { "w" } else { "b" }),
            IR::Call { dest } => write!(f, "call {}", dest),
            IR::Jmp { dest, short } => {
                write!(f, "jmp {}{}", if *short { "short " } else { "" }, dest)
            }
            IR::Ret { dest } => match dest {
                Some(dest) => write!(f, "ret {}", dest),
                None => write!(f, "ret"),
            },
            IR::Je { dest } => write!(f, "je {}", dest),
            IR::Jl { dest } => write!(f, "jl {}", dest),
            IR::Jle { dest } => write!(f, "jle {}", dest),
            IR::Jb { dest } => write!(f, "jb {}", dest),
            IR::Jbe { dest } => write!(f, "jbe {}", dest),
            IR::Jp { dest } => write!(f, "jp {}", dest),
            IR::Jo { dest } => write!(f, "jo {}", dest),
            IR::Js { dest } => write!(f, "js {}", dest),
            IR::Jne { dest } => write!(f, "jne {}", dest),
            IR::Jnl { dest } => write!(f, "jnl {}", dest),
            IR::Jnle { dest } => write!(f, "jnle {}", dest),
            IR::Jnb { dest } => write!(f, "jnb {}", dest),
            IR::Jnbe { dest } => write!(f, "jnbe {}", dest),
            IR::Jnp { dest } => write!(f, "jnp {}", dest),
            IR::Jno { dest } => write!(f, "jno {}", dest),
            IR::Jns { dest } => write!(f, "jns {}", dest),
            IR::Loop { dest } => write!(f, "loop {}", dest),
            IR::Loopz { dest } => write!(f, "loopz {}", dest),
            IR::Loopnz { dest } => write!(f, "loopnz {}", dest),
            IR::Jcxz { dest } => write!(f, "jcxz {}", dest),
            IR::Int { int_type } => {
                if *int_type == 3 {
                    write!(f, "int")
                } else {
                    write!(f, "int {:02x}", int_type)
                }
            }
            IR::Into => write!(f, "into"),
            IR::Iret => write!(f, "iret"),
            IR::Clc => write!(f, "clc"),
            IR::Cmc => write!(f, "cmc"),
            IR::Stc => write!(f, "stc"),
            IR::Cld => write!(f, "cld"),
            IR::Std => write!(f, "std"),
            IR::Cli => write!(f, "cli"),
            IR::Sti => write!(f, "sti"),
            IR::Hlt => write!(f, "hlt"),
            IR::Wait => write!(f, "wait"),
            IR::Esc { dest } => write!(f, "esc {}", dest),
            IR::Lock => write!(f, "lock"),
            IR::Undefined => write!(f, "(undefined)"),
        }
    }
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = match &self.base {
            Some(b) => format!("{}", b),
            None => "".to_string(),
        };
        let index = match &self.index {
            Some(i) => format!("+{}", i),
            None => "".to_string(),
        };

        // If only disp, convert to [imm]
        return if base.is_empty() && index.is_empty() && self.disp.is_some() {
            let value = match self.disp.as_ref().unwrap() {
                Displacement::Short(d) => *d as i16,
                Displacement::Long(d) => *d,
            };
            write!(f, "[{:0>4x}]", value)
        } else {
            write!(f, "[{}{}{}]", base, index, {
                match &self.disp {
                    Some(d) => {
                        if d.is_neg() {
                            format!("{}", d)
                        } else {
                            format!("+{}", d)
                        }
                    }
                    None => "".to_string(),
                }
            })
        };
    }
}

#[cfg(test)]
mod memory_display_test {
    use super::*;

    #[test]
    fn test_memory_display_no_displacement() {
        let memory = Memory {
            base: Some(Register::BX),
            index: None,
            disp: None,
        };
        assert_eq!(format!("{}", memory), "[bx]");
    }

    #[test]
    fn test_memory_display_with_8bits_displacement() {
        let memory = Memory {
            base: Some(Register::BX),
            index: None,
            disp: Some(Displacement::Short(0x5)),
        };
        assert_eq!(format!("{}", memory), "[bx+5]");
    }

    #[test]
    fn test_memory_display_with_16bits_displacement() {
        let memory = Memory {
            base: Some(Register::BX),
            index: None,
            disp: Some(Displacement::Long(0x1000)),
        };
        assert_eq!(format!("{}", memory), "[bx+1000]");
    }

    #[test]
    fn test_memory_display_with_base_index_displacement() {
        let memory = Memory {
            base: Some(Register::BX),
            index: Some(Register::SI),
            disp: Some(Displacement::Short(0x8)),
        };
        assert_eq!(format!("{}", memory), "[bx+si+8]");
    }

    #[test]
    fn test_memory_display_with_displacement_as_ea() {
        let memory = Memory {
            base: None,
            index: None,
            disp: Some(Displacement::Long(0x0010)),
        };
        assert_eq!(format!("{}", memory), "[0010]");
    }

    #[test]
    fn test_memory_display_with_signed_displacement() {
        let memory = Memory {
            base: Some(Register::BX),
            index: Some(Register::SI),
            disp: Some(Displacement::Long((0x89u8 as i8) as i16)),
        };
        assert_eq!(format!("{}", memory), "[bx+si-77]");
    }
}
