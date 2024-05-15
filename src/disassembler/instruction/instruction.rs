use crate::disassembler::instruction::Operand;

#[derive(Debug, PartialEq)]
pub enum IR {
    Mov { dest: Operand, src: Operand },
    Int { int_type: u8 },
    Add { dest: Operand, src: Operand },
    Sub { dest: Operand, src: Operand },
    Ssb { dest: Operand, src: Operand },
    Cmp { dest: Operand, src: Operand },
    And { dest: Operand, src: Operand },
    Or { dest: Operand, src: Operand },
    Xor { dest: Operand, src: Operand },
    Lea { dest: Operand, src: Operand },
    Lds { dest: Operand, src: Operand },
    Les { dest: Operand, src: Operand },
    Je { dest: Operand },
    Jl { dest: Operand },
    Jle { dest: Operand },
    Jb { dest: Operand },
    Jbe { dest: Operand },
    Jp { dest: Operand },
    Jo { dest: Operand },
    Js { dest: Operand },
    Jne { dest: Operand },
    Jnl { dest: Operand },
    Jnle { dest: Operand },
    Jnb { dest: Operand },
    Jnbe { dest: Operand },
    Jnp { dest: Operand },
    Jno { dest: Operand },
    Jns { dest: Operand },
    Loop { dest: Operand },
    Loopz { dest: Operand },
    Loopnz { dest: Operand },
    Jcxz { dest: Operand },
    Jmp { dest: Operand },
    Test { dest: Operand, src: Operand },
    Push { src: Operand },
    Call { dest: Operand },
    Hlt,
    Dec { dest: Operand },
    Shl { dest: Operand, src: Operand },
    Shr { dest: Operand, src: Operand },
    Sar { dest: Operand, src: Operand },
    Rol { dest: Operand, src: Operand },
    Ror { dest: Operand, src: Operand },
    Rcl { dest: Operand, src: Operand },
    Rcr { dest: Operand, src: Operand },
}

impl std::fmt::Display for IR {
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
            IR::Add { dest, src } => write!(f, "add {}, {}", dest, src),
            IR::Sub { dest, src } => write!(f, "sub {}, {}", dest, src),
            IR::Ssb { dest, src } => write!(f, "ssb {}, {}", dest, src),
            IR::Cmp { dest, src } => write!(f, "cmp {}, {}", dest, src),
            IR::And { dest, src } => write!(f, "and {}, {}", dest, src),
            IR::Or { dest, src } => write!(f, "or {}, {}", dest, src),
            IR::Xor { dest, src } => write!(f, "xor {}, {}", dest, src),
            IR::Lea { dest, src } => write!(f, "lea {}, {}", dest, src),
            IR::Lds { dest, src } => write!(f, "lds {}, {}", dest, src),
            IR::Les { dest, src } => write!(f, "les {}, {}", dest, src),
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
            IR::Jmp { dest } => write!(f, "jmp {}", dest),
            IR::Test { dest, src } => write!(f, "test {}, {}", dest, src),
            IR::Push { src } => write!(f, "push {}", src),
            IR::Call { dest } => write!(f, "call {}", dest),
            IR::Hlt => write!(f, "hlt"),
            IR::Dec { dest } => write!(f, "dec {}", dest),
            IR::Shl { dest, src } => write!(f, "shl {}, {}", dest, src),
            IR::Shr { dest, src } => write!(f, "shr {}, {}", dest, src),
            IR::Sar { dest, src } => write!(f, "sar {}, {}", dest, src),
            IR::Rol { dest, src } => write!(f, "rol {}, {}", dest, src),
            IR::Ror { dest, src } => write!(f, "ror {}, {}", dest, src),
            IR::Rcl { dest, src } => write!(f, "rcl {}, {}", dest, src),
            IR::Rcr { dest, src } => write!(f, "rcr {}, {}", dest, src),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub ir: IR,
    pub raw: Vec<u8>,
}

impl Instruction {
    pub fn new(ir: IR, raw: Vec<u8>) -> Self {
        Instruction { ir, raw }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t    {}",
            self.raw
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>(),
            self.ir
        )
    }
}
