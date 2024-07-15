use crate::disassembler::parser;
use crate::{
    disassembler::error::DisassemblerError,
    x86::{Address, Displacement, Instruction, Operand, Register, IR},
};
use pretty_assertions::assert_eq;

macro_rules! assert_parse {
    ($bytes:expr, $expected_result:expr) => {
        let bytes = $bytes;
        let expected_result = (
            Instruction::new($expected_result, bytes.to_vec()),
            bytes.len(),
        );
        assert_eq!(parser::parse_instruction(&bytes, 0), Ok(expected_result));
    };
}

#[test]
fn test_mov() {
    // imm reg
    assert_parse!(
        [0xbb, 0xFF, 0x00],
        IR::Mov {
            dest: Operand::Register(Register::BX),
            src: Operand::LongImmediate(0x00FF),
            byte: false,
        }
    );

    // imm r/m
    assert_parse!(
        [0xc7, 0x46, 0xfa, 0x00, 0x01],
        IR::Mov {
            dest: Operand::MemoryAddress(Address::new(
                Some(Register::BP),
                None,
                Some(Displacement::Long(-6)),
            )),
            src: Operand::LongImmediate(0x0100),
            byte: false,
        }
    );
}

#[test]
fn test_int() {
    assert_parse!([0xcc], IR::Int { int_type: 3 });
    assert_parse!([0xcd, 0x01], IR::Int { int_type: 1 });
}

#[test]
fn test_into() {
    assert_parse!([0xce], IR::Into);
}

#[test]
fn test_iret() {
    assert_parse!([0xcf], IR::Iret);
}

#[test]
fn test_add() {
    // r/m and reg
    assert_parse!(
        [0x00, 0x00],
        IR::Add {
            dest: Operand::MemoryAddress(Address::new(
                Some(Register::BX),
                Some(Register::SI),
                None
            )),
            src: Operand::Register(Register::AL),
        }
    );

    // Imm with r/m
    assert_parse!(
        [0x05, 0xc3, 0x14],
        IR::Add {
            dest: Operand::Register(Register::AX),
            src: Operand::LongImmediate(0x14c3)
        }
    );
}

#[test]
fn test_adc() {
    // r/m and reg
    assert_parse!(
        [0x11, 0xc9],
        IR::Adc {
            dest: Operand::Register(Register::CX),
            src: Operand::Register(Register::CX),
        }
    );
}

#[test]
fn test_sub() {
    // r/m and reg
    assert_parse!(
        [0x28, 0x00],
        IR::Sub {
            dest: Operand::MemoryAddress(Address::new(
                Some(Register::BX),
                Some(Register::SI),
                None
            )),
            src: Operand::Register(Register::AL),
        }
    );

    // Imm from r/m
    assert_parse!(
        [0x83, 0xeb, 0x14],
        IR::Sub {
            dest: Operand::Register(Register::BX),
            src: Operand::SignExtendedImmediate(0x14),
        }
    );

    // Imm from accumulator
    assert_parse!(
        [0x2d, 0x30, 0x00],
        IR::Sub {
            dest: Operand::Register(Register::AX),
            src: Operand::LongImmediate(0x0030),
        }
    );
}

#[test]
fn test_ssb() {
    // r/m and reg
    assert_parse!(
        [0x18, 0x00],
        IR::Ssb {
            dest: Operand::MemoryAddress(Address::new(
                Some(Register::BX),
                Some(Register::SI),
                None
            )),
            src: Operand::Register(Register::AL),
        }
    );

    // Imm from r/m
    assert_parse!(
        [0x83, 0x5e, 0xfe, 0x00],
        IR::Ssb {
            dest: Operand::MemoryAddress(Address::new(
                Some(Register::BP),
                None,
                Some(Displacement::Long(-2)),
            )),
            src: Operand::SignExtendedImmediate(0x00),
        }
    );
}

#[test]
fn test_inc() {
    // reg
    assert_parse!(
        [0x40],
        IR::Inc {
            dest: Operand::Register(Register::AX)
        }
    );

    // r/m
    assert_parse!(
        [0xff, 0x46, 0xf6],
        IR::Inc {
            dest: Operand::MemoryAddress(Address::new(
                Some(Register::BP),
                None,
                Some(Displacement::Long(-10)),
            )),
        }
    );
}

#[test]
fn test_cmp() {
    // r/m and reg
    assert_parse!(
        [0x38, 0x00],
        IR::Cmp {
            dest: Operand::MemoryAddress(Address::new(
                Some(Register::BX),
                Some(Register::SI),
                None
            )),
            src: Operand::Register(Register::AL),
            byte: true,
        }
    );

    // Imm with r/m
    assert_parse!(
        [0x83, 0x7c, 0x02, 0x00],
        IR::Cmp {
            dest: Operand::MemoryAddress(Address::new(
                Some(Register::SI),
                None,
                Some(Displacement::Long(0x2)),
            )),
            src: Operand::SignExtendedImmediate(0x00),
            byte: false,
        }
    );

    // Imm with accumulator
    assert_parse!(
        [0x3d, 0x01, 0x00],
        IR::Cmp {
            dest: Operand::Register(Register::AX),
            src: Operand::LongImmediate(0x01),
            byte: false,
        }
    );
}

#[test]
fn test_and() {
    // r/m and reg
    assert_parse!(
        [0x20, 0x00],
        IR::And {
            dest: Operand::MemoryAddress(Address::new(
                Some(Register::BX),
                Some(Register::SI),
                None
            )),
            src: Operand::Register(Register::AL),
        }
    );

    // Imm with r/m
    assert_parse!(
        [0x81, 0xe7, 0xfb, 0xff],
        IR::And {
            dest: Operand::Register(Register::DI),
            src: Operand::LongImmediate(0xfffb),
        }
    );
}

#[test]
fn test_or() {
    // r/m and reg
    assert_parse!(
        [0x8, 0x00],
        IR::Or {
            dest: Operand::MemoryAddress(Address::new(
                Some(Register::BX),
                Some(Register::SI),
                None
            )),
            src: Operand::Register(Register::AL),
        }
    );

    // Imm with r/m
    assert_parse!(
        [0x81, 0xcf, 0x01, 0x00],
        IR::Or {
            dest: Operand::Register(Register::DI),
            src: Operand::LongImmediate(0x01),
        }
    );
}

#[test]
fn test_xor() {
    // r/m and reg
    assert_parse!(
        [0x30, 0x00],
        IR::Xor {
            dest: Operand::MemoryAddress(Address::new(
                Some(Register::BX),
                Some(Register::SI),
                None
            )),
            src: Operand::Register(Register::AL),
        }
    );

    // Imm to r/m
    // assert_parse!(
    //     [0x80, 0x36, 0x02, 0x00],
    //     IR::Xor {
    //         dest: Operand::Memory(Memory::new(None, None, Some(Displacement::Long(0x2)),)),
    //         src: Operand::Immediate(0x00),
    //     }
    // );
}

#[test]
fn test_lea() {
    assert_parse!(
        [0x8D, 0x57, 0x02],
        IR::Lea {
            dest: Operand::Register(Register::DX),
            src: Operand::MemoryAddress(Address::new(
                Some(Register::BX),
                None,
                Some(Displacement::Long(0x2)),
            )),
        }
    );
}

#[test]
fn test_lds() {
    assert_parse!(
        [0xC5, 0x57, 0x02],
        IR::Lds {
            dest: Operand::Register(Register::DX),
            src: Operand::MemoryAddress(Address::new(
                Some(Register::BX),
                None,
                Some(Displacement::Long(0x2)),
            )),
        }
    );
}

#[test]
fn test_les() {
    assert_parse!(
        [0xC4, 0x57, 0x02],
        IR::Les {
            dest: Operand::Register(Register::DX),
            src: Operand::MemoryAddress(Address::new(
                Some(Register::BX),
                None,
                Some(Displacement::Long(0x2)),
            )),
        }
    );
}

#[test]
fn test_jmp() {
    // short segment
    assert_parse!(
        [0xeb, 0x0f],
        IR::Jmp {
            dest: Operand::Displacement(Displacement::Long(0x0f + 2)),
            short: true,
        }
    );

    // segment
    assert_parse!(
        [0xe9, 0x57, 0x02],
        IR::Jmp {
            dest: Operand::Displacement(Displacement::Long(0x0257 + 3)),
            short: false,
        }
    );

    // Indirect w/ segment
    assert_parse!(
        [0xff, 0xe3],
        IR::Jmp {
            dest: Operand::Register(Register::BX),
            short: false,
        }
    );

    // Indirect intersegment
    assert_parse!(
        [0xff, 0xe7],
        IR::Jmp {
            dest: Operand::Register(Register::DI),
            short: false,
        }
    );
}

#[test]
fn test_jnb() {
    assert_parse!(
        [0x73, 0x0f],
        IR::Jnb {
            dest: Operand::Displacement(Displacement::Long(0x0f + 2))
        }
    );
}

#[test]
fn test_test() {
    // Imm and r/m
    assert_parse!(
        [0xf6, 0xc3, 0x01],
        IR::Test {
            dest: Operand::Register(Register::BL),
            src: Operand::Immediate(0x01),
            byte: true
        }
    );

    // Imm and accumulator
    assert_parse!(
        [0xa8, 0x01],
        IR::Test {
            dest: Operand::Register(Register::AL),
            src: Operand::Immediate(0x01),
            byte: true
        }
    );
}

#[test]
fn test_push() {
    // reg
    assert_parse!(
        [0x50],
        IR::Push {
            src: Operand::Register(Register::AX)
        }
    );

    // r/m
    assert_parse!(
        [0xff, 0x76, 0x04],
        IR::Push {
            src: Operand::MemoryAddress(Address::new(
                Some(Register::BP),
                None,
                Some(Displacement::Long(0x4)),
            )),
        }
    );
}

#[test]
fn test_call() {
    // direct w/ segment
    assert_parse!(
        [0xe8, 0x57, 0x02],
        IR::Call {
            dest: Operand::Displacement(Displacement::Long(0x0257 + 3))
        }
    );

    // indirect w/ segment
    // intersegment
    assert_parse!(
        [0xff, 0xd3],
        IR::Call {
            dest: Operand::Register(Register::BX)
        }
    );
}

#[test]
fn test_dec() {
    // reg
    assert_parse!(
        [0x48],
        IR::Dec {
            dest: Operand::Register(Register::AX)
        }
    );

    // r/m
    assert_parse!(
        [0xff, 0x4e, 0xf4],
        IR::Dec {
            dest: Operand::MemoryAddress(Address::new(
                Some(Register::BP),
                None,
                Some(Displacement::Long(-0xc)),
            )),
        }
    );
}

#[test]
fn test_cli() {
    assert_parse!([0xfa], IR::Cli);
}

#[test]
fn test_sti() {
    assert_parse!([0xfb], IR::Sti);
}

#[test]
fn test_hlt() {
    assert_parse!([0xf4], IR::Hlt);
}

#[test]
fn test_wait() {
    assert_parse!([0x9b], IR::Wait);
}

#[test]
fn test_esc() {
    assert_parse!(
        [0xd9, 0xc0],
        IR::Esc {
            dest: Operand::Register(Register::AX)
        }
    );
}

#[test]
fn test_lock() {
    assert_parse!([0xf0], IR::Lock);
}

#[test]
fn test_shl() {
    assert_parse!(
        [0xd1, 0xe3],
        IR::Shl {
            dest: Operand::Register(Register::BX),
            src: Operand::Immediate(1)
        }
    );
}

#[test]
fn test_shr() {
    assert_parse!(
        [0xd1, 0xeb],
        IR::Shr {
            dest: Operand::Register(Register::BX),
            src: Operand::Immediate(1)
        }
    );
}

#[test]
fn test_sar() {
    assert_parse!(
        [0xd1, 0xff],
        IR::Sar {
            dest: Operand::Register(Register::DI),
            src: Operand::Immediate(1)
        }
    );
}

#[test]
fn test_rol() {
    assert_parse!(
        [0xd1, 0xc3],
        IR::Rol {
            dest: Operand::Register(Register::BX),
            src: Operand::Immediate(1)
        }
    );
}

#[test]
fn test_ror() {
    assert_parse!(
        [0xd1, 0xcb],
        IR::Ror {
            dest: Operand::Register(Register::BX),
            src: Operand::Immediate(1)
        }
    );
}

#[test]
fn test_rcl() {
    assert_parse!(
        [0xd1, 0xd3],
        IR::Rcl {
            dest: Operand::Register(Register::BX),
            src: Operand::Immediate(1)
        }
    );
}

#[test]
fn test_rcr() {
    assert_parse!(
        [0xd1, 0xdb],
        IR::Rcr {
            dest: Operand::Register(Register::BX),
            src: Operand::Immediate(1)
        }
    );
}

#[test]
fn test_pop() {
    // reg
    assert_parse!(
        [0x5b],
        IR::Pop {
            dest: Operand::Register(Register::BX)
        }
    );
}

#[test]
fn test_in() {
    // variable port
    assert_parse!(
        [0xec],
        IR::In {
            dest: Operand::Register(Register::AL),
            src: Operand::Register(Register::DX)
        }
    );

    // fixed port
    assert_parse!(
        [0xe4, 0x01],
        IR::In {
            dest: Operand::Register(Register::AL),
            src: Operand::Immediate(0x01)
        }
    );
}

#[test]
fn test_neg() {
    assert_parse!(
        [0xf7, 0xda],
        IR::Neg {
            dest: Operand::Register(Register::DX)
        }
    );
}

#[test]
fn test_clc() {
    assert_parse!([0xf8], IR::Clc);
}

#[test]
fn test_cmc() {
    assert_parse!([0xf5], IR::Cmc);
}

#[test]
fn test_stc() {
    assert_parse!([0xf9], IR::Stc);
}

#[test]
fn test_cld() {
    assert_parse!([0xfc], IR::Cld);
}

#[test]
fn test_std() {
    assert_parse!([0xfd], IR::Std);
}

#[test]
fn test_ret() {
    // Within segment
    assert_parse!([0xc3], IR::Ret { src: None });

    // Intersegment
    assert_parse!([0xcb], IR::Ret { src: None });
}

#[test]
fn test_mul() {
    assert_parse!(
        [0xf7, 0xe7],
        IR::Mul {
            dest: Operand::Register(Register::DI),
        }
    );
}

#[test]
fn test_imul() {
    assert_parse!(
        [0xf7, 0xef],
        IR::Imul {
            dest: Operand::Register(Register::DI),
        }
    );
}

#[test]
fn test_div() {
    assert_parse!(
        [0xf7, 0xf7],
        IR::Div {
            dest: Operand::Register(Register::DI),
        }
    );
}

#[test]
fn test_idiv() {
    assert_parse!(
        [0xf7, 0xff],
        IR::Idiv {
            dest: Operand::Register(Register::DI),
        }
    );
}

#[test]
fn test_not() {
    assert_parse!(
        [0xf7, 0xd7],
        IR::Not {
            dest: Operand::Register(Register::DI),
        }
    );
}

#[test]
fn test_cbw() {
    assert_parse!([0x98], IR::Cbw);
}

#[test]
fn test_cwb() {
    assert_parse!([0x99], IR::Cwd);
}

#[test]
fn test_rep() {
    // REP movs
    assert_parse!(
        [0xf2, 0xa5],
        IR::Rep {
            z: false,
            string_ir: Box::new(IR::Movs { word: true }),
        }
    );
}

#[test]
fn test_movs() {
    assert_parse!([0xa5], IR::Movs { word: true });
}

#[test]
fn test_cmps() {
    assert_parse!([0xa6], IR::Cmps { word: false });
}

#[test]
fn test_scas() {
    assert_parse!([0xaf], IR::Scas { word: true });
}

#[test]
fn test_lods() {
    assert_parse!([0xad], IR::Lods { word: true });
}

#[test]
fn test_stos() {
    assert_parse!([0xaa], IR::Stos { word: false });
}

// #[test]
// fn test_invalid_opcode() {
//     // Test parsing an invalid opcode
//     let bytes = [0xFA];
//     assert_eq!(
//         parse_instruction(&bytes, 0),
//         Err(ParseError::InvalidOpcode(0xFA))
//     );
// }

#[test]
fn test_unexpected_eof() {
    let bytes = [0b10110000];
    assert_eq!(
        parser::parse_instruction(&bytes, 0),
        Err(DisassemblerError::UnexpectedEOF)
    );
}
