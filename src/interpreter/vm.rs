use super::memory::Memory;
use super::register_set::RegisterSet;
use crate::utils::{min, HexdumpFormatter};
use crate::x86::{Address, Displacement};
use crate::{minix::Program, x86::IR};

use log::trace;

// Opcode implementations
mod opcodes;
use opcodes::OpcodeExecutable;

struct VM {
    // cpu
    pub ip: u16,
    // memory
    pub text: Memory,
    pub data: Memory,
    // registers
    pub regs: RegisterSet,
    pub flags: u16,
}

impl From<Program> for VM {
    fn from(program: Program) -> Self {
        let text = Memory::from(program.text_segment.data);
        let data = Memory::from(program.data_segment.data);
        let regs = RegisterSet::new();
        let flags = 0;
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
    fn fetch(&self) -> &[u8];
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
    fn fetch(&self) -> &[u8] {
        let ip = self.ip;
        self.text
            .read_bytes(ip, min(MAX_INSTRUCTION_SIZE, self.text.len() - ip as usize))
    }

    fn decode(&self, chunk: &[u8]) -> (IR, usize) {
        let (ins, ir_len) = match crate::disassembler::parse_instruction(chunk, self.ip.into()) {
            Ok((instruction, bytes_consumed)) => (instruction, bytes_consumed),
            // Err(DisassemblerError::UnexpectedEOF) => {
            //     (Instruction::new(IR::Undefined, text.to_vec()), text.len())
            // }
            Err(e) => panic!("Error decoding instruction: {:?}", e),
        };
        trace!("[DECODE] IR:\n{:?}\t{}", HexdumpFormatter(&ins.raw), ins.ir);
        (ins.ir, ir_len)
    }

    fn execute(&mut self, ir: IR) {
        trace!("[EXECUTE] {:?}", ir);
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
            // pop, push, ...
            _ => panic!("Not implemented"),
        }
    }

    fn run(&mut self) {
        loop {
            trace!("\nVM STATE: {}", self);
            let ir = self.fetch();
            let (decoded_ir, ir_len) = self.decode(ir);
            self.execute(decoded_ir);

            // Increment the instruction pointer (ip) appropriately
            self.ip += ir_len as u16;
        }
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
}

// Display for VM
// IP:
// FLAGS:
// TEXT: HEXDUMP
// DATA: HEXDUMP
//
// REG
// AX:
// BX:
// ...
impl std::fmt::Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "IP: {:04x}", self.ip)?;
        writeln!(f, "FLAGS: {:04x}", self.flags)?;
        writeln!(f, "TEXT:")?;
        write!(f, "{:?}", HexdumpFormatter(&self.text.data))?;
        writeln!(f, "DATA:")?;
        writeln!(f, "{:?}", HexdumpFormatter(&self.data.data))?;
        writeln!(f, "REGS:")?;
        writeln!(f, "{:?}", self.regs)?;
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
// }
