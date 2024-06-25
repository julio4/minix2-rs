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

impl From<Program> for VM {
    fn from(program: Program) -> Self {
        let text = Memory::from(program.text_segment.data);

        let size = 0x1000;
        let mut data = Memory::new(size);
        data.write_bytes(0, &program.data_segment.data);

        let mut regs = RegisterSet::new();
        regs.set(Register::SP, (data.len() - 2) as i16);
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
    fn run(&mut self);
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
            IR::Jnb { dest } => {
                self.jnb(dest);
            }
            IR::Jne { dest } => {
                self.jne(dest);
            }
            IR::Je { dest } => {
                self.je(dest);
            }
            IR::Test { dest, src, byte: _ } => {
                self.test(dest, src);
            }
            IR::Push { src } => {
                self.push(src);
            }
            IR::Call { dest } => {
                self.call(dest);
            }
            IR::In { dest, src } => {
                self.in_(dest, src);
            }
            IR::Loopnz { dest } => {
                self.loopnz(dest);
            }
            IR::Or { dest, src } => {
                self.or(dest, src);
            }
            _ => panic!("{}: Not implemented", ir),
        }
    }

    fn run(&mut self) {
        trace!(" AX   BX   CX   DX   SP   BP   SI   DI        IP");
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
                    for flag in Flag::iter() {
                        flags.push_str(&format!("{}", self.flags.get(flag) as u8));
                    }
                    format!(
                        "{} {} {:04x}:{}",
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

            self.execute(decoded_ir);

            // Increment the instruction pointer (ip) appropriately
            self.ip += ir_len as u16;
        }
        // trace!("Execution finished:\n{}", self);
    }
}

pub trait Interpretable {
    fn interpret(self);
}

impl Interpretable for Program {
    fn interpret(self) {
        VM::from(self).run()
    }
}

impl VM {
    fn get_effective_address(&self, address: Address) -> u16 {
        let base = match address.base {
            Some(b) => self.regs.get(b),
            None => 0,
        };
        let index = match address.index {
            Some(i) => self.regs.get(i),
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

    fn into_value(&self, operand: Operand) -> i16 {
        match operand {
            Operand::Register(reg) => self.regs.get(reg),
            Operand::Immediate(value) => value as i16,
            Operand::LongImmediate(value) => value as i16,
            Operand::SignExtendedImmediate(value) => value as i16,
            Operand::MemoryAddress(address) => {
                let ea = self.get_effective_address(address);
                self.data.read_word(ea) as i16
            }
            Operand::Displacement(value) => self.ip.wrapping_add(value.into()) as i16,
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

// #[cfg(test)]
// mod tests {
// }
