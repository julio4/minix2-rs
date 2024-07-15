use std::process::exit;

use super::VM;
use crate::{
    interpreter::flag_set::Flag,
    x86::{Operand, Register},
};
use log::trace;

pub trait OpcodeExecutable {
    fn mov(&mut self, dest: Operand, src: Operand, byte: bool);
    fn int(&mut self, int_type: u8);
    fn add(&mut self, dest: Operand, src: Operand);
    fn xor(&mut self, dest: Operand, src: Operand);
    fn lea(&mut self, dest: Operand, src: Operand);
    fn cmp(&mut self, dest: Operand, src: Operand);
    fn jmp(&mut self, dest: Operand);
    fn jnb(&mut self, dest: Operand);
    fn jne(&mut self, dest: Operand);
    fn je(&mut self, dest: Operand);
    fn jl(&mut self, dest: Operand);
    fn jnl(&mut self, dest: Operand);
    fn test(&mut self, dest: Operand, src: Operand);
    fn push(&mut self, src: Operand);
    fn pop(&mut self, dest: Operand);
    fn call(&mut self, dest: Operand);
    fn ret(&mut self, src: Option<Operand>);
    fn in_(&mut self, dest: Operand, src: Operand);
    fn loop_(&mut self, dest: Operand);
    fn loopz(&mut self, dest: Operand);
    fn loopnz(&mut self, dest: Operand);
    fn or(&mut self, dest: Operand, src: Operand);
    fn sub(&mut self, dest: Operand, src: Operand);
    fn dec(&mut self, dest: Operand);
}

impl OpcodeExecutable for VM {
    fn mov(&mut self, dest: Operand, src: Operand, _byte: bool) {
        let src_value = self.read_value(&src);
        self.write_value(&dest, src_value as u16);
    }
    fn int(&mut self, int_type: u8) {
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
                        trace!("<exit({})>", message_source);
                        exit(message_source as i32);
                    }
                    4 => {
                        // _sendrec
                        let content_len = self.data.read_word(message_struct_ea + 6);
                        let content_ea = self.data.read_word(message_struct_ea + 10);
                        trace!(
                            "<write({}, {:#04x}, {})>",
                            message_source,
                            content_ea,
                            content_len
                        );
                        // set AX to 0
                        self.regs.set(Register::AX, 0);
                        // Return nb of bytes written
                        self.data.write_word(message_struct_ea + 2, content_len);

                        let content = self.data.read_bytes(content_ea, content_len as usize);
                        print!("{}", String::from_utf8_lossy(content));
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
    fn add(&mut self, dest: Operand, src: Operand) {
        let src_value = self.read_value(&src);
        let dest_value = self.read_value(&dest);
        let (result, overflow) = dest_value.overflowing_add(src_value);

        self.write_value(&dest, result as u16);

        self.flags.set(Flag::Overflow, overflow);
        self.flags.set(Flag::Carry, result < dest_value);
        self.flags.set_szp(result);
        self.flags
            .set(Flag::Aux, (dest_value & 0xf) + (src_value & 0xf) > 0xf);
    }
    fn xor(&mut self, dest: Operand, src: Operand) {
        let src_value = self.read_value(&src);
        let target_value = self.read_value(&dest);
        let result = target_value ^ src_value;

        self.write_value(&dest, result as u16);

        // Clear
        self.flags.clear(Flag::Overflow);
        self.flags.clear(Flag::Carry);
        // SF, ZF and PF based on result
        self.flags.set_szp(result);
    }
    fn lea(&mut self, dest: Operand, src: Operand) {
        let address = match src {
            Operand::MemoryAddress(address) => address,
            _ => panic!("Invalid operand"),
        };
        match dest {
            Operand::Register(reg) => {
                let ea = self.get_effective_address(address);
                self.regs.set(reg, ea);
            }
            _ => panic!("Invalid operand"),
        }
    }
    fn cmp(&mut self, dest: Operand, src: Operand) {
        let src_value = self.read_value(&src);
        let dest_value = self.read_value(&dest);
        let (result, overflow) = dest_value.overflowing_sub(src_value);

        self.flags.set(Flag::Carry, dest_value < src_value);
        self.flags.set(Flag::Overflow, overflow);
        self.flags.set_szp(result);
        self.flags
            .set(Flag::Aux, (dest_value & 0xf) < (src_value & 0xf));
    }
    fn jmp(&mut self, dest: Operand) {
        self.ip = self.read_value(&dest) as u16;
    }
    fn jnb(&mut self, dest: Operand) {
        if !self.flags.get(Flag::Carry) {
            self.ip = self.read_value(&dest) as u16;
        }
    }
    fn jne(&mut self, dest: Operand) {
        if !self.flags.get(Flag::Zero) {
            self.ip = self.read_value(&dest) as u16;
        }
    }
    fn je(&mut self, dest: Operand) {
        if self.flags.get(Flag::Zero) {
            self.ip = self.read_value(&dest) as u16;
        }
    }
    fn jl(&mut self, dest: Operand) {
        if self.flags.get(Flag::Sign) != self.flags.get(Flag::Overflow) {
            self.ip = self.read_value(&dest) as u16;
        }
    }
    fn jnl(&mut self, dest: Operand) {
        if self.flags.get(Flag::Sign) == self.flags.get(Flag::Overflow) {
            self.ip = self.read_value(&dest) as u16;
        }
    }
    fn test(&mut self, dest: Operand, src: Operand) {
        let src_value = self.read_value(&src);
        let dest_value = self.read_value(&dest);
        let result = dest_value & src_value;

        // Clear
        self.flags.clear(Flag::Carry);
        self.flags.clear(Flag::Overflow);
        // SF, ZF, PF
        self.flags.set_szp(result);
    }
    fn sub(&mut self, dest: Operand, src: Operand) {
        let src_value = self.read_value(&src);
        let dest_value = self.read_value(&dest);
        let (result, overflow) = dest_value.overflowing_sub(src_value);

        self.write_value(&dest, result as u16);

        // OF, CF
        self.flags.set(Flag::Overflow, overflow);
        self.flags.set(Flag::Carry, dest_value > src_value);
        // SF, ZF, PF
        self.flags.set_szp(result);
    }
    fn push(&mut self, src: Operand) {
        let value = self.read_value(&src) as u16;
        let ea = self.regs.get(Register::SP).wrapping_sub(2) as u16;
        self.data.write_word(ea, value as u16);
        self.regs.set(Register::SP, ea);
    }
    fn pop(&mut self, dest: Operand) {
        let ea = self.regs.get(Register::SP) as u16;
        let value = self.data.read_word(ea);
        self.regs.set(Register::SP, ea.wrapping_add(2));
        match dest {
            Operand::Register(reg) => self.regs.set(reg, value),
            _ => unimplemented!(),
        }
    }
    fn call(&mut self, dest: Operand) {
        let value = self.read_value(&dest) as u16;
        let ea = self.regs.get(Register::SP).wrapping_sub(2) as u16;
        self.data.write_word(ea, self.ip);
        self.regs.set(Register::SP, ea);
        self.ip = value;
    }
    fn ret(&mut self, src: Option<Operand>) {
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
    }
    fn in_(&mut self, dest: Operand, src: Operand) {
        let _port = self.read_value(&src) as u16;
        let value = 0x42;
        match dest {
            Operand::Register(reg) => self.regs.set(reg, value),
            _ => unimplemented!(),
        }
    }
    fn loop_(&mut self, dest: Operand) {
        let value = self.read_value(&dest) as u16;
        let cx = self.regs.get(Register::CX) - 1;
        self.regs.set(Register::CX, cx);
        if cx != 0 {
            self.ip = value;
        }
    }
    fn loopz(&mut self, dest: Operand) {
        let value = self.read_value(&dest) as u16;
        let cx = self.regs.get(Register::CX) - 1;
        self.regs.set(Register::CX, cx);
        if cx != 0 && self.flags.get(Flag::Zero) {
            self.ip = value;
        }
    }
    fn loopnz(&mut self, dest: Operand) {
        let value = self.read_value(&dest) as u16;
        let cx = self.regs.get(Register::CX) - 1;
        self.regs.set(Register::CX, cx);
        if cx != 0 && !self.flags.get(Flag::Zero) {
            self.ip = value;
        }
    }
    fn or(&mut self, dest: Operand, src: Operand) {
        let src_value = self.read_value(&src);
        let dest_value = self.read_value(&dest);
        let result = dest_value | src_value;

        self.write_value(&dest, result as u16);

        // Clear
        self.flags.clear(Flag::Overflow);
        self.flags.clear(Flag::Carry);
        // SF, ZF and PF based on result
        self.flags.set_szp(result);
    }
    fn dec(&mut self, dest: Operand) {
        let dest_value = self.read_value(&dest);
        let result = dest_value.wrapping_sub(1);

        self.write_value(&dest, result as u16);

        // Clear
        self.flags.clear(Flag::Overflow);
        // SF, ZF and PF based on result
        self.flags.set_szp(result);
    }
}
