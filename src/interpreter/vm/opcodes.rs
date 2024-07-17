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
    fn neg(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors>;
    fn cwd(&mut self) -> Result<(), OpcodeExecErrors>;
    fn div(&mut self, src: Operand) -> Result<(), OpcodeExecErrors>;
    fn xchg(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors>;
    fn sar(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors>;
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
                let message_type_ea = message_struct_ea + 2;
                let i1_ea = message_struct_ea + 4;
                let i2_ea = message_struct_ea + 6;
                let i3_ea = message_struct_ea + 8;
                let m1_p1_ea = message_struct_ea + 10;
                let m2_p1_ea = message_struct_ea + 18;

                let message_type = self.data.read_word(message_type_ea);
                match message_type {
                    1 => {
                        // exit(status)
                        // status = m1_i1
                        let status = self.data.read_word(i1_ea);
                        self.trace(format!("\n<exit({})>", status).as_str());
                        exec_exit(status as i32);
                        return Err(OpcodeExecErrors::ExitCatch);
                    }
                    4 => {
                        // write(fd, buffer, nbytes)
                        // fd = m1_i1
                        // nbytes = m1_i2
                        // buffer = m1_p1
                        let nbytes = self.data.read_word(i2_ea);
                        let buffer = self.data.read_word(m1_p1_ea);

                        // set AX to 0
                        self.regs.set(Register::AX, 0);
                        // Return nb of bytes written
                        let return_value = nbytes;
                        self.data.write_word(message_type_ea, nbytes);

                        let content =
                            String::from_utf8_lossy(self.data.read_bytes(buffer, nbytes as usize));
                        self.trace(
                            format!(
                                "\n<write({}, {:#06x}, {}){} => {}>",
                                1, buffer, nbytes, content, return_value
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
                        // brk(addr)
                        // addr = m1_p1
                        let addr = self.data.read_word(m1_p1_ea);

                        // return 0
                        self.data.write_word(message_type_ea, 0);
                        self.trace(format!("\n<brk({:#06x}) => {}>", addr, 0).as_str());
                        Ok(())
                    }
                    54 => {
                        // ioctl(fd, request, data)
                        // fd = TTY_LINE = DEVICE = m2_i1
                        // request = TTY_REQUEST = COUNT = m2_i3
                        // data = ADDRESS = m2_p1
                        let fd = self.data.read_word(i1_ea);
                        let request = self.data.read_word(i3_ea);
                        let data = self.data.read_word(m2_p1_ea);
                        self.trace(
                            format!("\n<ioctl({}, {:#04x}, {:#04x})>", fd, request, data).as_str(),
                        );

                        // return ffea (not sure why yet)
                        self.data.write_word(message_type_ea, 0xffea);

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
                Operand::LongImmediate(value) => value,
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
    fn neg(&mut self, dest: Operand) -> Result<(), OpcodeExecErrors> {
        let dest_value = self.read_value(&dest);
        let (result, overflow) = dest_value.overflowing_neg();

        self.write_value(&dest, result as u16);

        // CF = 0 if dest is 0
        self.flags.set(Flag::Carry, dest_value != 0);
        self.flags.set(Flag::Overflow, overflow);
        // SF, ZF and PF based on result
        self.flags.set_szp(result);
        Ok(())
    }
    fn cwd(&mut self) -> Result<(), OpcodeExecErrors> {
        let ax = self.regs.get(Register::AX) as i16;
        let dx = if ax < 0 { 0xffff } else { 0x0000 };
        self.regs.set(Register::DX, dx);
        Ok(())
    }
    fn div(&mut self, src: Operand) -> Result<(), OpcodeExecErrors> {
        let src_value = self.read_value(&src);
        let ax = self.regs.get(Register::AX) as u16;
        let dx = self.regs.get(Register::DX) as u16;
        let dividend = (dx as u32) << 16 | ax as u32;
        let quotient = dividend / src_value as u32;
        let remainder = dividend % src_value as u32;

        if quotient > 0xffff {
            return Err(OpcodeExecErrors::DivideError);
        }

        self.regs.set(Register::AX, quotient as u16);
        self.regs.set(Register::DX, remainder as u16);
        Ok(())
    }
    fn xchg(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors> {
        let dest_value = self.read_value(&dest);
        let src_value = self.read_value(&src);

        self.write_value(&dest, src_value as u16);
        self.write_value(&src, dest_value as u16);
        Ok(())
    }
    fn sar(&mut self, dest: Operand, src: Operand) -> Result<(), OpcodeExecErrors> {
        let src_value = self.read_value(&src);
        let dest_value = self.read_value(&dest);
        let result = (dest_value as i16 >> src_value) as u16;

        self.write_value(&dest, result as u16);

        // CF flag contains last bit shifted out
        self.flags
            .set(Flag::Carry, (dest_value & (1 << (src_value - 1))) != 0);
        // SF, ZF and PF based on result
        self.flags.set_szp(result as i16);
        Ok(())
    }
}
