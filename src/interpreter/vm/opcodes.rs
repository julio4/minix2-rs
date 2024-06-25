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
    fn jnb(&mut self, dest: Operand);
    fn jne(&mut self, dest: Operand);
    fn je(&mut self, dest: Operand);
    fn test(&mut self, dest: Operand, src: Operand);
    fn push(&mut self, src: Operand);
    fn call(&mut self, dest: Operand);
    fn in_(&mut self, dest: Operand, src: Operand);
    fn loopnz(&mut self, dest: Operand);
    fn or(&mut self, dest: Operand, src: Operand);
}

impl OpcodeExecutable for VM {
    fn mov(&mut self, dest: Operand, src: Operand, _byte: bool) {
        let value = self.into_value(src);
        match dest {
            Operand::Register(reg) => self.regs.set(reg, value),
            Operand::MemoryAddress(address) => {
                let ea = self.get_effective_address(address);
                self.data.write_bytes(ea, &(value as u16).to_le_bytes());
            }
            _ => unimplemented!(),
        }
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
        let value = self.into_value(src);
        if value == 0 {
            return;
        }
        match dest {
            Operand::Register(reg) => {
                let current = self.regs.get(reg);
                self.regs.set(reg, current.wrapping_add(value));
            }
            Operand::MemoryAddress(address) => {
                let ea = self.get_effective_address(address);
                let current_value = self.data.read_word(ea);
                self.data.write_bytes(
                    ea,
                    &(current_value.wrapping_add(value.try_into().unwrap()) as u16).to_le_bytes(),
                );
            }
            _ => unimplemented!(),
        }
    }
    fn xor(&mut self, dest: Operand, src: Operand) {
        let value = self.into_value(src);
        match dest {
            Operand::Register(reg) => {
                let current = self.regs.get(reg);
                self.regs.set(reg, current ^ value);
            }
            _ => unimplemented!(),
        }
    }
    fn lea(&mut self, dest: Operand, src: Operand) {
        let address = match src {
            Operand::MemoryAddress(address) => address,
            _ => panic!("Invalid operand"),
        };
        match dest {
            Operand::Register(reg) => {
                let ea = self.get_effective_address(address);
                self.regs.set(reg, ea as i16);
            }
            _ => panic!("Invalid operand"),
        }
    }
    fn cmp(&mut self, dest: Operand, src: Operand) {
        let value = self.into_value(src);
        let current = self.into_value(dest);

        let result = current.wrapping_sub(value);
        self.flags.set(Flag::Zero, result == 0);
        self.flags.set(Flag::Sign, result < 0);
        self.flags.set(Flag::Carry, current < value);
        self.flags.set(
            Flag::Overflow,
            (current < 0 && value > 0 && result > 0) || (current > 0 && value < 0 && result < 0),
        );
    }
    fn jnb(&mut self, dest: Operand) {
        let value = self.into_value(dest) as u16;
        if !self.flags.get(Flag::Carry) {
            self.ip = value;
        }
    }
    fn jne(&mut self, dest: Operand) {
        let value = self.into_value(dest) as u16;
        if !self.flags.get(Flag::Zero) {
            self.ip = value;
        }
    }
    fn je(&mut self, dest: Operand) {
        let value = self.into_value(dest) as u16;
        if self.flags.get(Flag::Zero) {
            self.ip = value;
        }
    }
    fn test(&mut self, dest: Operand, src: Operand) {
        let value = self.into_value(src);
        match dest {
            Operand::Register(reg) => {
                let current = self.regs.get(reg);
                let result = current & value;
                self.flags.set(Flag::Zero, result == 0);
                self.flags.set(Flag::Sign, result < 0);
                self.flags.set(Flag::Carry, false);
                self.flags.set(Flag::Overflow, false);
            }
            _ => unimplemented!(),
        }
    }
    fn push(&mut self, src: Operand) {
        let value = self.into_value(src) as u16;
        let ea = self.regs.get(Register::SP).wrapping_sub(2) as u16;
        self.data.write_word(ea, value as u16);
    }
    fn call(&mut self, dest: Operand) {
        let value = self.into_value(dest) as u16;
        let ea = self.regs.get(Register::SP).wrapping_sub(2) as u16;
        self.data.write_word(ea, self.ip);
        self.regs.set(Register::SP, ea as i16);
        self.ip = value;
    }
    fn in_(&mut self, dest: Operand, src: Operand) {
        let _port = self.into_value(src) as u16;
        let value = 0x42;
        match dest {
            Operand::Register(reg) => self.regs.set(reg, value as i16),
            _ => unimplemented!(),
        }
    }
    fn loopnz(&mut self, dest: Operand) {
        let value = self.into_value(dest) as u16;
        let cx = self.regs.get(Register::CX) - 1;
        self.regs.set(Register::CX, cx);
        if cx != 0 && !self.flags.get(Flag::Zero) {
            self.ip = value;
        }
    }
    fn or(&mut self, dest: Operand, src: Operand) {
        let value = self.into_value(src);
        match dest {
            Operand::Register(reg) => {
                let current = self.regs.get(reg);
                self.regs.set(reg, current | value);
            }
            _ => unimplemented!(),
        }
    }
}
