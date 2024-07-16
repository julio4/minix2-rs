use super::{VirtualMemory, VM};
use crate::{
    interpreter::{error::OpcodeExecErrors, flag_set::Flag},
    x86::{Operand, Register},
};

pub trait OpcodeExecutable {
    fn mov(&mut self, dest: Operand, src: Operand, byte: bool) -> Result<(), OpcodeExecErrors>;
    fn int(&mut self, int_type: u8) -> Result<(), OpcodeExecErrors>;
    fn add(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors>;
    fn xor(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors>;
    fn lea(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors>;
    fn cmp(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors>;
    fn jmp(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn jb(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn jbe(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn jnb(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn jne(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn je(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn jl(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn jle(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn jnl(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn jnle(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn jnbe(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn test(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors>;
    fn push(&mut self, src: Operand) -> Result<(), OpcodeExecErrors>;
    fn pop(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn call(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn ret(&mut self, src: Option<Operand>) -> Result<(), OpcodeExecErrors>;
    fn in_(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors>;
    fn loop_(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn loopz(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn loopnz(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn or(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors>;
    fn sub(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors>;
    fn dec(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn cbw(&mut self) -> Result<(), OpcodeExecErrors>;
    fn inc(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn and(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors>;
    fn shl(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors>;
}

// Small trick to not exit the program when running tests
#[cfg(not(test))]
fn exec_exit(code: i32) {
    std::process::exit(code);
}

#[cfg(test)]
fn exec_exit(_code: i32) {}

impl OpcodeExecutable for VM {
    fn mov(&mut self, dest: Operand, src: Operand, _byte: bool) -> Result<(), OpcodeExecErrors> {
        let src_value = self.read_value(&src);
        self.write_value(&dest, src_value as u16);
        Ok(())
    }
    fn int(&mut self, int_type: u8) -> Result<(), OpcodeExecErrors> {
        match int_type {
            // Syscalls
            0x20 => {
                // struct message {
                //     uint16_t m_source;
                //     uint16_t m_type;
                //     union m_u;
                // };
                let message_struct_ea = self.regs.get(Register::BX) as u16;
                let message_source = self.data.read_word(message_struct_ea);
                let message_type = self.data.read_word(message_struct_ea + 2);

                match message_type {
                    1 => {
                        self.trace(format!("\n<exit({})>", message_source).as_str());
                        exec_exit(message_source as i32);
                        return Err(OpcodeExecErrors::ExitCatch);
                    }
                    4 => {
                        // _sendrec
                        let content_len = self.data.read_word(message_struct_ea + 6);
                        let content_ea = self.data.read_word(message_struct_ea + 10);
                        // set AX to 0
                        self.regs.set(Register::AX, 0);
                        // Return nb of bytes written
                        self.data.write_word(message_struct_ea + 2, content_len);

                        let content = String::from_utf8_lossy(
                            self.data.read_bytes(content_ea, content_len as usize),
                        );
                        self.trace(
                            format!(
                                "\n<write({}, {:#06x}, {}){} => {}>",
                                1, content_ea, content_len, content, content_len
                            )
                            .as_str(),
                        );
                        // if not trace
                        if !self.trace {
                            print!("{}", content);
                        }
                        Ok(())
                    }
                    17 => {
                        // BRK
                        self.trace(format!("\n<brk({})>", message_source).as_str());
                        Ok(())
                    }
                    54 => {
                        // IOCTL
                        let content_len = self.data.read_word(message_struct_ea + 6);
                        let content_ea = self.data.read_word(message_struct_ea + 10);
                        self.trace(
                            format!(
                                "<ioctl({}, {:#04x}, {:#04x})>",
                                message_source, content_ea, content_len,
                            )
                            .as_str(),
                        );
                        // What to do?
                        // set AX to 0
                        self.regs.set(Register::AX, 0);
                        Ok(())
                    }
                    _ => Err(OpcodeExecErrors::UnimplementedSyscall(
                        message_type as usize,
                    )),
                }
            }
            _ => Err(OpcodeExecErrors::UnimplementedInterrupt(int_type as usize)),
        }
    }
    fn add(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors> {
        let src_value = self.read_value(&src);
        let dest_value = self.read_value(&dest);
        let (result, overflow) = dest_value.overflowing_add(src_value);

        self.write_value(&dest, result as u16);

        self.flags.set(Flag::Overflow, overflow);
        self.flags.set(Flag::Carry, result < dest_value);
        self.flags.set_szp(result);
        self.flags
            .set(Flag::Aux, (dest_value & 0xf) + (src_value & 0xf) > 0xf);
        Ok(())
    }
    fn xor(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors> {
        let src_value = self.read_value(&src);
        let target_value = self.read_value(&dest);
        let result = target_value ^ src_value;

        self.write_value(&dest, result as u16);

        // Clear
        self.flags.clear(Flag::Overflow);
        self.flags.clear(Flag::Carry);
        // SF, ZF and PF based on result
        self.flags.set_szp(result);
        Ok(())
    }
    fn lea(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors> {
        let address = match src {
            Operand::MemoryAddress(address) => address,
            _ => panic!("Invalid operand"),
        };
        address.trace(self);
        match dest {
            Operand::Register(reg) => {
                let ea = address.get_effective_address(self);
                self.regs.set(reg, ea);
            }
            _ => panic!("Invalid operand"),
        }
        Ok(())
    }
    fn cmp(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors> {
        let src_value = self.read_value(&src);
        let dest_value = self.read_value(&dest);
        let (result, overflow) = dest_value.overflowing_sub(src_value);

        self.flags.set(Flag::Carry, dest_value < src_value);
        self.flags.set(Flag::Overflow, overflow);
        self.flags.set_szp(result);
        self.flags
            .set(Flag::Aux, (dest_value & 0xf) < (src_value & 0xf));
        Ok(())
    }
    fn jmp(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        self.ip = self.read_value(&dest) as u16;
        Ok(())
    }
    fn jb(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        if self.flags.get(Flag::Carry) {
            self.ip = self.read_value(&dest) as u16;
        }
        Ok(())
    }
    fn jbe(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        if self.flags.get(Flag::Carry) || self.flags.get(Flag::Zero) {
            self.ip = self.read_value(&dest) as u16;
        }
        Ok(())
    }
    fn jnb(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        if !self.flags.get(Flag::Carry) {
            self.ip = self.read_value(&dest) as u16;
        }
        Ok(())
    }
    fn jne(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        if !self.flags.get(Flag::Zero) {
            self.ip = self.read_value(&dest) as u16;
        }
        Ok(())
    }
    fn je(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        if self.flags.get(Flag::Zero) {
            self.ip = self.read_value(&dest) as u16;
        }
        Ok(())
    }
    fn jl(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        if self.flags.get(Flag::Sign) != self.flags.get(Flag::Overflow) {
            self.ip = self.read_value(&dest) as u16;
        }
        Ok(())
    }
    fn jle(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        if self.flags.get(Flag::Zero)
            || self.flags.get(Flag::Sign) != self.flags.get(Flag::Overflow)
        {
            self.ip = self.read_value(&dest) as u16;
        }
        Ok(())
    }
    fn jnl(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        if self.flags.get(Flag::Sign) == self.flags.get(Flag::Overflow) {
            self.ip = self.read_value(&dest) as u16;
        }
        Ok(())
    }
    fn jnle(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        if !self.flags.get(Flag::Zero)
            && self.flags.get(Flag::Sign) == self.flags.get(Flag::Overflow)
        {
            self.ip = self.read_value(&dest) as u16;
        }
        Ok(())
    }
    fn jnbe(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        if !self.flags.get(Flag::Carry) && !self.flags.get(Flag::Zero) {
            self.ip = self.read_value(&dest) as u16;
        }
        Ok(())
    }
    fn test(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors> {
        let src_value = self.read_value(&src);
        let dest_value = self.read_value(&dest);
        let result = dest_value & src_value;

        // Clear
        self.flags.clear(Flag::Carry);
        self.flags.clear(Flag::Overflow);
        // SF, ZF, PF
        self.flags.set_szp(result);
        Ok(())
    }
    fn sub(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors> {
        let src_value = self.read_value(&src);
        let dest_value = self.read_value(&dest);
        let (result, overflow) = dest_value.overflowing_sub(src_value);

        self.write_value(&dest, result as u16);

        // OF, CF
        self.flags.set(Flag::Overflow, overflow);
        self.flags.set(Flag::Carry, dest_value > src_value);
        // SF, ZF, PF
        self.flags.set_szp(result);
        Ok(())
    }
    fn push(&mut self, src: Operand) -> Result<(), OpcodeExecErrors> {
        let value = self.read_value(&src) as u16;
        let ea = self.regs.get(Register::SP).wrapping_sub(2) as u16;
        self.data.write_word(ea, value as u16);
        self.regs.set(Register::SP, ea);
        Ok(())
    }
    fn pop(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        let ea = self.regs.get(Register::SP) as u16;
        let value = self.data.read_word(ea);
        self.regs.set(Register::SP, ea.wrapping_add(2));
        match dest {
            Operand::Register(reg) => self.regs.set(reg, value),
            _ => unimplemented!(),
        }
        Ok(())
    }
    fn call(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        let value = self.read_value(&dest) as u16;
        let ea = self.regs.get(Register::SP).wrapping_sub(2) as u16;
        self.data.write_word(ea, self.ip);
        self.regs.set(Register::SP, ea);
        self.ip = value;
        Ok(())
    }
    fn ret(&mut self, src: Option<Operand>) -> Result<(), OpcodeExecErrors> {
        let ea = self.regs.get(Register::SP) as u16;
        let value = self.data.read_word(ea);
        let released_bytes = match src {
            Some(src) => match src {
                Operand::Immediate(value) => value as u16,
                _ => panic!("Invalid operand"),
            },
            None => 0,
        };
        self.regs
            .set(Register::SP, ea.wrapping_add(2 + released_bytes));
        self.ip = value;
        Ok(())
    }
    fn in_(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors> {
        let _port = self.read_value(&src) as u16;
        let value = 0x42;
        match dest {
            Operand::Register(reg) => self.regs.set(reg, value),
            _ => unimplemented!(),
        }
        Ok(())
    }
    fn loop_(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        let value = self.read_value(&dest) as u16;
        let cx = self.regs.get(Register::CX) - 1;
        self.regs.set(Register::CX, cx);
        if cx != 0 {
            self.ip = value;
        }
        Ok(())
    }
    fn loopz(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        let value = self.read_value(&dest) as u16;
        let cx = self.regs.get(Register::CX) - 1;
        self.regs.set(Register::CX, cx);
        if cx != 0 && self.flags.get(Flag::Zero) {
            self.ip = value;
        }
        Ok(())
    }
    fn loopnz(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        let value = self.read_value(&dest) as u16;
        let cx = self.regs.get(Register::CX) - 1;
        self.regs.set(Register::CX, cx);
        if cx != 0 && !self.flags.get(Flag::Zero) {
            self.ip = value;
        }
        Ok(())
    }
    fn or(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors> {
        let src_value = self.read_value(&src);
        let dest_value = self.read_value(&dest);
        let result = dest_value | src_value;

        self.write_value(&dest, result as u16);

        // Clear
        self.flags.clear(Flag::Overflow);
        self.flags.clear(Flag::Carry);
        // SF, ZF and PF based on result
        self.flags.set_szp(result);
        Ok(())
    }
    fn dec(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        let dest_value = self.read_value(&dest);
        let result = dest_value.wrapping_sub(1);

        self.write_value(&dest, result as u16);

        // Clear
        self.flags.clear(Flag::Overflow);
        // SF, ZF and PF based on result
        self.flags.set_szp(result);
        Ok(())
    }
    fn cbw(&mut self) -> Result<(), OpcodeExecErrors> {
        let al = self.regs.get(Register::AL) as i8;
        self.regs.set(Register::AX, (al as i16).try_into().unwrap());
        Ok(())
    }
    fn inc(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        let dest_value = self.read_value(&dest);
        let result = dest_value.wrapping_add(1);

        self.write_value(&dest, result as u16);

        // Clear
        self.flags.clear(Flag::Overflow);
        // SF, ZF and PF based on result
        self.flags.set_szp(result);
        Ok(())
    }
    fn and(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors> {
        let src_value = self.read_value(&src);
        let dest_value = self.read_value(&dest);
        let result = dest_value & src_value;

        self.write_value(&dest, result as u16);

        // Clear
        self.flags.clear(Flag::Overflow);
        self.flags.clear(Flag::Carry);
        // SF, ZF and PF based on result
        self.flags.set_szp(result);
        Ok(())
    }
    fn shl(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors> {
        let src_value = self.read_value(&src);
        let dest_value = self.read_value(&dest);
        let result = dest_value << src_value;

        self.write_value(&dest, result as u16);

        // CF flag contains last bit shifted out
        self.flags
            .set(Flag::Carry, (dest_value & (1 << (16 - src_value))) != 0);
        // OF flag set only for 1-bit shifts
        if src_value == 1 {
            self.flags
                .set(Flag::Overflow, (dest_value & 0x8000u16 as i16) != 0);
        }
        // SF, ZF and PF based on result
        self.flags.set_szp(result);
        Ok(())
    }
}
