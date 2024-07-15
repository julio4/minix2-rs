use super::error::InterpreterError;
use super::flag_set::FlagSet;
use super::memory::Memory;
use super::register_set::RegisterSet;
use crate::interpreter::flag_set::Flag;
use crate::utils::{min, HexdumpFormatter};
use crate::x86::{Address, Displacement, Operand, Register};
use crate::{minix::Program, x86::IR};

use log::trace;

// Opcode implementations
mod opcodes;
use opcodes::OpcodeExecutable;

#[allow(dead_code)]
struct VM {
    // cpu
    pub ip: u16,
    // memory
    pub text: Memory,
    pub data: Memory,
    // registers, flags
    pub regs: RegisterSet,
    pub flags: FlagSet,
}

impl Default for VM {
    fn default() -> Self {
        let text = Memory::new(0);
        let data = Memory::new(0x1000);

        let regs = RegisterSet::new();
        let flags = FlagSet::new();
        let ip = 0;
        VM {
            ip,
            text,
            data,
            regs,
            flags,
        }
    }
}

impl From<Program> for VM {
    fn from(program: Program) -> Self {
        let text = Memory::from(program.text_segment.data);

        let mut data = Memory::new(0x10000);
        data.write_bytes(0, &program.data_segment.data);

        let mut regs = RegisterSet::new();
        regs.set(Register::SP, 0xffd0);
        data.write_bytes(0xffd0, &0x1u16.to_le_bytes());

        let flags = FlagSet::new();
        let ip = 0;
        VM {
            ip,
            text,
            data,
            regs,
            flags,
        }
    }
}

pub trait VmIrExecutable: OpcodeExecutable {
    // Fetch the next chunk from text memory from ip
    fn fetch(&self) -> Option<&[u8]>;
    // Decode the fetched chunk to an IR
    fn decode(&self, chunk: &[u8]) -> (IR, usize);
    // Execute the decoded instruction
    // + Implicit store
    fn execute(&mut self, ir: IR);
    // Run the VM from the program loaded in memory
    fn run(&mut self) -> Result<(), InterpreterError>;
}

const MAX_INSTRUCTION_SIZE: usize = 15;
impl VmIrExecutable for VM {
    fn fetch(&self) -> Option<&[u8]> {
        let ip = self.ip;
        if self.text.len() <= ip as usize {
            return None;
        }
        Some(
            self.text
                .read_bytes(ip, min(MAX_INSTRUCTION_SIZE, self.text.len() - ip as usize)),
        )
    }

    fn decode(&self, chunk: &[u8]) -> (IR, usize) {
        let (ins, ir_len) = match crate::disassembler::parse_instruction(chunk, self.ip.into()) {
            Ok((instruction, bytes_consumed)) => (instruction, bytes_consumed),
            // Err(DisassemblerError::UnexpectedEOF) => {}
            Err(e) => panic!("Error decoding instruction: {:?}", e),
        };
        (ins.ir, ir_len)
    }

    fn execute(&mut self, ir: IR) {
        match ir {
            IR::Mov { dest, src, byte } => {
                self.mov(dest, src, byte);
            }
            IR::Int { int_type } => {
                self.int(int_type);
            }
            IR::Add { dest, src } => {
                self.add(dest, src);
            }
            IR::Xor { dest, src } => {
                self.xor(dest, src);
            }
            IR::Lea { dest, src } => {
                self.lea(dest, src);
            }
            IR::Cmp { dest, src, byte: _ } => {
                self.cmp(dest, src);
            }
            IR::Jmp { dest, short: _ } => {
                self.jmp(dest);
            }
            IR::Jnb { dest } => {
                self.jnb(dest);
            }
            IR::Jne { dest } => {
                self.jne(dest);
            }
            IR::Je { dest } => {
                self.je(dest);
            }
            IR::Jl { dest } => {
                self.jl(dest);
            }
            IR::Jnl { dest } => {
                self.jnl(dest);
            }
            IR::Test { dest, src, byte: _ } => {
                self.test(dest, src);
            }
            IR::Push { src } => {
                self.push(src);
            }
            IR::Pop { dest } => {
                self.pop(dest);
            }
            IR::Call { dest } => {
                self.call(dest);
            }
            IR::Ret { src } => {
                self.ret(src);
            }
            IR::In { dest, src } => {
                self.in_(dest, src);
            }
            IR::Loop { dest } => {
                self.loop_(dest);
            }
            IR::Loopz { dest } => {
                self.loopz(dest);
            }
            IR::Loopnz { dest } => {
                self.loopnz(dest);
            }
            IR::Or { dest, src } => {
                self.or(dest, src);
            }
            IR::Sub { dest, src } => {
                self.sub(dest, src);
            }
            IR::Dec { dest } => {
                self.dec(dest);
            }
            IR::Cbw => {
                self.cbw();
            }
            _ => panic!("{}: Not implemented", ir),
        }
    }

    fn run(&mut self) -> Result<(), InterpreterError> {
        trace!(" AX   BX   CX   DX   SP   BP   SI   DI  FLAGS IP");
        let mut cycle_count = 0;
        while let Some(ir) = self.fetch() {
            let (decoded_ir, ir_len) = self.decode(ir);

            // Trace with format:
            //  AX   BX   CX   DX   SP   BP   SI   DI  FLAGS IP
            // 0000 0000 0000 0000 0000 0000 0000 0000 ---- 0000:bb0000     mov bx, 000
            trace!(
                "{}       \t{}",
                {
                    let mut regs = String::new();
                    for reg in vec![
                        Register::AX,
                        Register::BX,
                        Register::CX,
                        Register::DX,
                        Register::SP,
                        Register::BP,
                        Register::SI,
                        Register::DI,
                    ] {
                        regs.push_str(&format!("{:04x} ", self.regs.get(reg)));
                    }
                    let mut flags = String::new();
                    // if self.flags.get(Flag::Parity) {
                    //     flags.push('P');
                    // } else {
                    //     flags.push('-');
                    // }
                    flags.push('-'); // ?
                    if self.flags.get(Flag::Sign) {
                        flags.push('S');
                    } else {
                        flags.push('-');
                    }
                    if self.flags.get(Flag::Zero) {
                        flags.push('Z');
                    } else {
                        flags.push('-');
                    }
                    if self.flags.get(Flag::Carry) {
                        flags.push('C');
                    } else {
                        flags.push('-');
                    }

                    format!(
                        "{}{} {:04x}:{}",
                        regs,
                        flags,
                        self.ip,
                        &ir[..ir_len]
                            .iter()
                            .map(|b| format!("{:02x}", b))
                            .collect::<String>(),
                    )
                },
                decoded_ir
            );

            // Increment the instruction pointer (ip) appropriately
            self.ip += ir_len as u16;

            self.execute(decoded_ir);

            // Check cycle count
            cycle_count += 1;
            if cycle_count > 10000 {
                return Err(InterpreterError::CycleLimitExceeded);
            }
        }

        // trace!("Execution finished:\n{}", self);
        Ok(())
    }
}

pub trait Interpretable {
    fn interpret(self) -> Result<(), InterpreterError>;
}

impl Interpretable for Program {
    fn interpret(self) -> Result<(), InterpreterError> {
        VM::from(self).run()
    }
}

impl VM {
    fn get_effective_address(&self, address: Address) -> u16 {
        let base = match address.base {
            Some(b) => self.regs.get(b) as i16,
            None => 0,
        };
        let index = match address.index {
            Some(i) => self.regs.get(i) as i16,
            None => 0,
        };
        let disp = match address.disp {
            Some(d) => match d {
                Displacement::Short(d) => d as i16,
                Displacement::Long(d) => d,
            },
            None => 0,
        };

        base.wrapping_add(index).wrapping_add(disp) as u16
    }

    fn read_value(&self, operand: &Operand) -> i16 {
        match operand {
            Operand::Register(reg) => self.regs.get(*reg) as i16,
            Operand::Immediate(value) => *value as i16,
            Operand::LongImmediate(value) => *value as i16,
            Operand::SignExtendedImmediate(value) => *value as i16,
            Operand::MemoryAddress(address) => {
                let ea = self.get_effective_address(*address);
                let ev = self.data.read_word(ea) as i16;
                trace!(";[{:04x}]{:04x}", ea, ev);
                ev
            }
            Operand::Displacement(value) => match value {
                Displacement::Short(d) => *d as i16,
                Displacement::Long(d) => *d,
            },
            // DEBUG: `self.ip.wrapping_add((*value).into()) as i16``, if not this, then check disasm `call Displacement`, e.g. ir `e80500`, instead of `call Imm`
        }
    }

    fn write_value(&mut self, operand: &Operand, value: u16) {
        match operand {
            Operand::Register(reg) => self.regs.set(*reg, value),
            Operand::MemoryAddress(address) => {
                let ea = self.get_effective_address(*address);
                self.data.write_word(ea, value);
            }
            _ => panic!("Invalid operand"),
        }
    }
}

impl std::fmt::Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "IP: {:04x}", self.ip)?;
        writeln!(f, "FLAGS: {}", self.flags)?;
        writeln!(f, "TEXT:")?;
        write!(f, "{:?}", HexdumpFormatter(&self.text.data))?;
        writeln!(f, "DATA:")?;
        writeln!(f, "{:?}", HexdumpFormatter(&self.data.data))?;
        writeln!(f, "REGS:")?;
        writeln!(f, "{}", self.regs)?;
        Ok(())
    }
}
