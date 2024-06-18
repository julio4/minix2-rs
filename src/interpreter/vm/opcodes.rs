use super::VM;
use crate::x86::{Operand, Register};
use log::trace;

pub trait OpcodeExecutable {
    fn mov(&mut self, dest: Operand, src: Operand, byte: bool);
    fn int(&mut self, int_type: u8);
    fn add(&mut self, dest: Operand, src: Operand);
}

impl OpcodeExecutable for VM {
    fn mov(&mut self, dest: Operand, src: Operand, _byte: bool) {
        let value = match src {
            Operand::Register(reg) => self.regs.get(reg),
            Operand::Immediate(value) => value as i16,
            Operand::LongImmediate(value) => value as i16,
            _ => unimplemented!(),
        };
        match dest {
            Operand::Register(reg) => self.regs.set(reg, value),
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
        let value = match src {
            Operand::Register(reg) => self.regs.get(reg),
            Operand::Immediate(value) => value as i16,
            Operand::LongImmediate(value) => value as i16,
            _ => unimplemented!(),
        };
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
}
