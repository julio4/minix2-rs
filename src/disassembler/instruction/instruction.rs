use crate::disassembler::instruction::Operand;

#[derive(Debug, PartialEq)]
pub enum IR {
    Mov {
        dest: Operand,
        src: Operand,
        byte: bool,
    },
    Push {
        src: Operand,
    },
    Pop {
        dest: Operand,
    },
    Xchg {
        dest: Operand,
        src: Operand,
    },
    In {
        dest: Operand,
        src: Operand,
    },
    Out {
        dest: Operand,
        src: Operand,
    },
    Xlat,
    Lea {
        dest: Operand,
        src: Operand,
    },
    Lds {
        dest: Operand,
        src: Operand,
    },
    Les {
        dest: Operand,
        src: Operand,
    },
    Lahf,
    Sahf,
    Pushf,
    Popf,
    Add {
        dest: Operand,
        src: Operand,
    },
    Adc {
        dest: Operand,
        src: Operand,
    },
    Inc {
        dest: Operand,
    },
    Aaa,
    Baa,
    Sub {
        dest: Operand,
        src: Operand,
    },
    Ssb {
        dest: Operand,
        src: Operand,
    },
    Dec {
        dest: Operand,
    },
    Neg {
        dest: Operand,
    },
    Cmp {
        dest: Operand,
        src: Operand,
    },
    Aas,
    Das,
    Mul {
        dest: Operand,
    },
    Imul {
        dest: Operand,
    },
    Aam,
    Div {
        dest: Operand,
    },
    Idiv {
        dest: Operand,
    },
    Aad,
    Cbw,
    Cwd,
    Not {
        dest: Operand,
    },
    Shl {
        dest: Operand,
        src: Operand,
    },
    Shr {
        dest: Operand,
        src: Operand,
    },
    Sar {
        dest: Operand,
        src: Operand,
    },
    Rol {
        dest: Operand,
        src: Operand,
    },
    Ror {
        dest: Operand,
        src: Operand,
    },
    Rcl {
        dest: Operand,
        src: Operand,
    },
    Rcr {
        dest: Operand,
        src: Operand,
    },
    And {
        dest: Operand,
        src: Operand,
    },
    Test {
        dest: Operand,
        src: Operand,
        byte: bool,
    },
    Or {
        dest: Operand,
        src: Operand,
    },
    Xor {
        dest: Operand,
        src: Operand,
    },
    Rep {
        z: bool,
        string_ir: Box<IR>,
    },
    Movs {
        word: bool,
    },
    Cmps,
    Scas,
    Lods,
    Stos,
    Call {
        dest: Operand,
    },
    Jmp {
        dest: Operand,
        short: bool,
    },
    Ret,
    Je {
        dest: Operand,
    },
    Jl {
        dest: Operand,
    },
    Jle {
        dest: Operand,
    },
    Jb {
        dest: Operand,
    },
    Jbe {
        dest: Operand,
    },
    Jp {
        dest: Operand,
    },
    Jo {
        dest: Operand,
    },
    Js {
        dest: Operand,
    },
    Jne {
        dest: Operand,
    },
    Jnl {
        dest: Operand,
    },
    Jnle {
        dest: Operand,
    },
    Jnb {
        dest: Operand,
    },
    Jnbe {
        dest: Operand,
    },
    Jnp {
        dest: Operand,
    },
    Jno {
        dest: Operand,
    },
    Jns {
        dest: Operand,
    },
    Loop {
        dest: Operand,
    },
    Loopz {
        dest: Operand,
    },
    Loopnz {
        dest: Operand,
    },
    Jcxz {
        dest: Operand,
    },
    Int {
        int_type: u8,
    },
    Into,
    Iret,
    Clc,
    Cmc,
    Stc,
    Cld,
    Std,
    Cli,
    Sti,
    Hlt,
    Wait,
    Esc,
    Lock,
    Undefined,
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
                    _ => "",
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
            IR::Cmp { dest, src } => write!(f, "cmp {}, {}", dest, src),
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
                    _ => "",
                },
                dest,
                src
            ),
            IR::Or { dest, src } => write!(f, "or {}, {}", dest, src),
            IR::Xor { dest, src } => write!(f, "xor {}, {}", dest, src),
            IR::Rep { z: _, string_ir } => write!(f, "rep {}", string_ir),
            IR::Movs { word } => write!(f, "movs{}", if *word { "w" } else { "b" }),
            IR::Cmps => write!(f, "cmps"),
            IR::Scas => write!(f, "scas"),
            IR::Lods => write!(f, "lods"),
            IR::Stos => write!(f, "stos"),
            IR::Call { dest } => write!(f, "call {}", dest),
            IR::Jmp { dest, short } => {
                write!(f, "jmp {}{}", if *short { "short " } else { "" }, dest)
            }
            IR::Ret => write!(f, "ret"),
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
            IR::Esc => write!(f, "esc"),
            IR::Lock => write!(f, "lock"),
            IR::Undefined => write!(f, "(undefined)"),
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
            "{:<14}{}",
            self.raw
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>(),
            self.ir
        )
    }
}
