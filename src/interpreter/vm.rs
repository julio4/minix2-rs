use core::str;

use super::error::{InterpreterError, OpcodeExecErrors};
use super::flag_set::FlagSet;
use super::memory::Memory;
use super::register_set::RegisterSet;
use crate::interpreter::flag_set::Flag;
use crate::utils::{min, HexdumpFormatter};
use crate::x86::{Address, Displacement, Operand, Register};
use crate::{minix::Program, x86::IR};

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
    // configs
    pub trace: bool,
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
            trace: false,
        }
    }
}

impl From<Program> for VM {
    fn from(program: Program) -> Self {
        let text = Memory::from(program.text_segment.data);

        let mut data = Memory::new(0x10000);
        data.write_bytes(0, &program.data_segment.data);

        let mut regs = RegisterSet::new();
        regs.set(Register::SP, 0xffda);
        // Not sure for this
        data.write_bytes(0xffda, &0x1u16.to_le_bytes());
        data.write_bytes(0xffdc, &0xffe4u16.to_le_bytes());

        let flags = FlagSet::new();
        let ip = 0;
        VM {
            ip,
            text,
            data,
            regs,
            flags,
            trace: false,
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
    fn execute(&mut self, ir: IR) -> Result<(), OpcodeExecErrors>;
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

    fn execute(&mut self, ir: IR) -> Result<(), OpcodeExecErrors> {
        match ir {
            IR::Mov { dest, src, byte } => self.mov(dest, src, byte),
            IR::Int { int_type } => self.int(int_type),
            IR::Add { dest, src } => self.add(dest, src),
            IR::Xor { dest, src } => self.xor(dest, src),
            IR::Lea { dest, src } => self.lea(dest, src),
            IR::Cmp { dest, src, byte: _ } => self.cmp(dest, src),
            IR::Jmp { dest, short: _ } => self.jmp(dest),
            IR::Jb { dest } => self.jb(dest),
            IR::Jbe { dest } => self.jbe(dest),
            IR::Jnb { dest } => self.jnb(dest),
            IR::Jne { dest } => self.jne(dest),
            IR::Je { dest } => self.je(dest),
            IR::Jl { dest } => self.jl(dest),
            IR::Jle { dest } => self.jle(dest),
            IR::Jnl { dest } => self.jnl(dest),
            IR::Jnle { dest } => self.jnle(dest),
            IR::Jnbe { dest } => self.jnbe(dest),
            IR::Test { dest, src, byte: _ } => self.test(dest, src),
            IR::Push { src } => self.push(src),
            IR::Pop { dest } => self.pop(dest),
            IR::Call { dest } => self.call(dest),
            IR::Ret { src } => self.ret(src),
            IR::In { dest, src } => self.in_(dest, src),
            IR::Loop { dest } => self.loop_(dest),
            IR::Loopz { dest } => self.loopz(dest),
            IR::Loopnz { dest } => self.loopnz(dest),
            IR::Or { dest, src } => self.or(dest, src),
            IR::Sub { dest, src } => self.sub(dest, src),
            IR::Dec { dest } => self.dec(dest),
            IR::Cbw => self.cbw(),
            IR::Inc { dest } => self.inc(dest),
            IR::And { dest, src } => self.and(dest, src),
            IR::Shl { dest, src } => self.shl(dest, src),
            IR::Neg { dest } => self.neg(dest),
            IR::Cwd => self.cwd(),
            IR::Div { dest } => self.div(dest),
            IR::Xchg { dest, src } => self.xchg(dest, src),
            IR::Sar { dest, src } => self.sar(dest, src),
            IR::Hlt => Ok(()), // we handle it directly in the run loop
            _ => panic!("{}: Not implemented", ir),
        }
    }

    fn run(&mut self) -> Result<(), InterpreterError> {
        self.trace(" AX   BX   CX   DX   SP   BP   SI   DI  FLAGS IP\n");
        let mut cycle_count = 0;
        while let Some(ir) = self.fetch() {
            let (decoded_ir, ir_len) = self.decode(ir);

            match decoded_ir {
                IR::Hlt => {
                    return Ok(());
                }
                _ => {}
            }

            self.trace(
                format!(
                    "{:<62} {}",
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
                )
                .as_str(),
            );

            // Increment the instruction pointer (ip) appropriately
            self.ip += ir_len as u16;

            match self.execute(decoded_ir) {
                Ok(_) => {}
                Err(e) => match e {
                    OpcodeExecErrors::ExitCatch => {
                        self.trace("\n");
                        return Ok(());
                    }
                    _ => {
                        return Err(InterpreterError::OpcodeExecutionError(e));
                    }
                },
            };

            self.trace("\n");
            // Check cycle count
            cycle_count += 1;
            if cycle_count > 999999 {
                return Err(InterpreterError::CycleLimitExceeded);
            }
        }
        Ok(())
    }
}

pub trait Interpretable {
    fn interpret(self, trace: bool, args: Vec<String>) -> Result<(), InterpreterError>;
}

impl Interpretable for Program {
    fn interpret(self, trace: bool, args: Vec<String>) -> Result<(), InterpreterError> {
        let mut vm = VM::from(self);
        vm.set_args(args);
        vm.set_trace(trace);
        vm.run()
    }
}

trait VirtualMemory {
    fn get_effective_address(&self, vm: &VM) -> u16;
    fn read_value(&self, vm: &VM) -> i16;
    fn write_value(&self, vm: &mut VM, value: u16);
    fn trace(&self, vm: &VM);
}

impl VirtualMemory for Address {
    fn get_effective_address(&self, vm: &VM) -> u16 {
        let base = match self.base {
            Some(b) => vm.regs.get(b) as i16,
            None => 0,
        };
        let index = match self.index {
            Some(i) => vm.regs.get(i) as i16,
            None => 0,
        };
        let disp = match self.disp {
            Some(d) => match d {
                Displacement::Short(d) => d as i16,
                Displacement::Long(d) => d,
            },
            None => 0,
        };

        base.wrapping_add(index).wrapping_add(disp) as u16
    }

    fn read_value(&self, vm: &VM) -> i16 {
        let ea = self.get_effective_address(vm);
        let ev = vm.data.read_word(ea) as i16;
        vm.trace(format!(" ;[{:04x}]{:04x}", ea, ev).as_str());
        ev
    }

    fn write_value(&self, vm: &mut VM, value: u16) {
        let ea = self.get_effective_address(vm);
        let ev = vm.data.read_word(ea);
        vm.trace(format!(" ;[{:04x}]{:04x}", ea, ev).as_str());
        vm.data.write_word(ea, value);
    }

    fn trace(&self, vm: &VM) {
        let ea = self.get_effective_address(vm);
        let ev = vm.data.read_word(ea);
        vm.trace(format!(" ;[{:04x}]{:04x}", ea, ev).as_str());
    }
}

impl VM {
    fn read_value(&self, operand: &Operand) -> i16 {
        match operand {
            Operand::Register(reg) => self.regs.get(*reg) as i16,
            Operand::Immediate(value) => *value as i16,
            Operand::LongImmediate(value) => *value as i16,
            Operand::SignExtendedImmediate(value) => *value as i16,
            Operand::MemoryAddress(address) => address.read_value(self),
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
            Operand::MemoryAddress(address) => address.write_value(self, value),
            _ => panic!("Invalid operand"),
        }
    }

    fn set_trace(&mut self, trace: bool) {
        self.trace = trace;
    }

    fn trace(&self, str: &str) {
        if self.trace {
            print!("{}", str);
        }
    }

    fn set_args(&mut self, args: Vec<String>) {
        let mut argv_pointers = Vec::new();

        let mut total_length = 0;
        for arg in &args {
            total_length += arg.len() + 1; // each argument string + null terminator
        }

        let initial_sp = self.regs.get(Register::SP);
        self.regs
            .set(Register::SP, initial_sp.wrapping_sub(total_length as u16));

        let mut current_sp = initial_sp.wrapping_sub(total_length as u16);
        for arg in &args {
            argv_pointers.push(current_sp); // Record the pointer to this argument

            // Copy the argument string and null terminator into memory
            for byte in arg.bytes() {
                self.data.write_bytes(current_sp, &[byte]);
                current_sp = current_sp.wrapping_add(1);
            }
            self.data.write_bytes(current_sp, &[0]); // Null terminator
            current_sp = current_sp.wrapping_add(1);
        }

        self.regs
            .set(Register::SP, self.regs.get(Register::SP).wrapping_sub(2));
        self.data.write_word(self.regs.get(Register::SP), 0);

        for &pointer in argv_pointers.iter() {
            self.regs
                .set(Register::SP, self.regs.get(Register::SP).wrapping_sub(2));
            self.data.write_word(self.regs.get(Register::SP), pointer);
        }

        let argc = argv_pointers.len() as u16;
        self.regs
            .set(Register::SP, self.regs.get(Register::SP).wrapping_sub(2));
        self.data.write_word(self.regs.get(Register::SP), argc);
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

pub fn vm_interpret(args: Vec<String>) {
    // Args validation
    if args.len() < 2 {
        println!("Usage: {} <binary file> [-m] additional_args", args[0]);
        return;
    }

    // Logger
    let trace = args.len() > 2 && args[2] == "-m";
    // remove the first arg
    let parsed_args = args[1..].to_vec();

    // Open file
    let file = std::fs::File::open(&args[1]).unwrap();
    let program = Program::from_file(file).unwrap();

    // Interpreter
    program.interpret(trace, parsed_args).unwrap();
}
