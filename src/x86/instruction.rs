use super::Operand;

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
        byte: bool,
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
    Cmps {
        word: bool,
    },
    Scas {
        word: bool,
    },
    Lods {
        word: bool,
    },
    Stos {
        word: bool,
    },
    Call {
        dest: Operand,
    },
    Jmp {
        dest: Operand,
        short: bool,
    },
    Ret {
        dest: Option<Operand>,
    },
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
    Esc {
        dest: Operand,
    },
    Lock,
    Undefined,
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
