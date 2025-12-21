use super::Cpu;
use crate::memory::Memory;

impl Cpu {
    pub(super) fn execute_cb_instruction(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        match opcode {
            0x00..=0x07 => self.rlc_r(opcode, memory),
            0x08..=0x0F => self.rrc_r(opcode, memory),
            0x10..=0x17 => self.rl_r(opcode, memory),
            0x18..=0x1F => self.rr_r(opcode, memory),
            0x20..=0x27 => self.sla_r(opcode, memory),
            0x28..=0x2F => self.sra_r(opcode, memory),
            0x38..=0x3F => self.srl_r(opcode, memory),
            0x40..=0x7F => self.bit_n_r(opcode, memory),
            0x80..=0xBF => self.res_n_r(opcode, memory),
            0xC0..=0xFF => self.set_n_r(opcode, memory),
            _ => {
                eprintln!("Unknown CB opcode: 0x{:02X} at PC: 0x{:04X}", opcode, self.pc - 2);
                8
            }
        }
    }

    fn rlc_r(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let reg = opcode & 0x07;
        let val = self.read_reg(reg, memory);
        let bit7 = val >> 7;
        let result = (val << 1) | bit7;
        self.write_reg(reg, result, memory);

        self.set_flag_c(bit7 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_pv(result.count_ones() % 2 == 0);
        self.set_flag_x((result & 0x20) != 0);
        self.set_flag_y((result & 0x08) != 0);

        if reg == 6 { 15 } else { 8 }
    }

    fn rrc_r(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let reg = opcode & 0x07;
        let val = self.read_reg(reg, memory);
        let bit0 = val & 1;
        let result = (val >> 1) | (bit0 << 7);
        self.write_reg(reg, result, memory);

        self.set_flag_c(bit0 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_pv(result.count_ones() % 2 == 0);
        self.set_flag_x((result & 0x20) != 0);
        self.set_flag_y((result & 0x08) != 0);

        if reg == 6 { 15 } else { 8 }
    }

    fn rl_r(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let reg = opcode & 0x07;
        let old_carry = if self.get_flag_c() { 1 } else { 0 };
        let val = self.read_reg(reg, memory);
        let bit7 = val >> 7;
        let result = (val << 1) | old_carry;
        self.write_reg(reg, result, memory);

        self.set_flag_c(bit7 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_pv(result.count_ones() % 2 == 0);
        self.set_flag_x((result & 0x20) != 0);
        self.set_flag_y((result & 0x08) != 0);

        if reg == 6 { 15 } else { 8 }
    }

    fn rr_r(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let reg = opcode & 0x07;
        let old_carry = if self.get_flag_c() { 0x80 } else { 0 };
        let val = self.read_reg(reg, memory);
        let bit0 = val & 1;
        let result = (val >> 1) | old_carry;
        self.write_reg(reg, result, memory);

        self.set_flag_c(bit0 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_pv(result.count_ones() % 2 == 0);
        self.set_flag_x((result & 0x20) != 0);
        self.set_flag_y((result & 0x08) != 0);

        if reg == 6 { 15 } else { 8 }
    }

    fn sla_r(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let reg = opcode & 0x07;
        let val = self.read_reg(reg, memory);
        let bit7 = val >> 7;
        let result = val << 1;
        self.write_reg(reg, result, memory);

        self.set_flag_c(bit7 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_pv(result.count_ones() % 2 == 0);
        self.set_flag_x((result & 0x20) != 0);
        self.set_flag_y((result & 0x08) != 0);

        if reg == 6 { 15 } else { 8 }
    }

    fn sra_r(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let reg = opcode & 0x07;
        let val = self.read_reg(reg, memory);
        let bit7 = val & 0x80;
        let bit0 = val & 1;
        let result = (val >> 1) | bit7;
        self.write_reg(reg, result, memory);

        self.set_flag_c(bit0 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_pv(result.count_ones() % 2 == 0);
        self.set_flag_x((result & 0x20) != 0);
        self.set_flag_y((result & 0x08) != 0);

        if reg == 6 { 15 } else { 8 }
    }

    fn srl_r(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let reg = opcode & 0x07;
        let val = self.read_reg(reg, memory);
        let bit0 = val & 1;
        let result = val >> 1;
        self.write_reg(reg, result, memory);

        self.set_flag_c(bit0 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_z(result == 0);
        self.set_flag_s(false);
        self.set_flag_pv(result.count_ones() % 2 == 0);
        self.set_flag_x((result & 0x20) != 0);
        self.set_flag_y((result & 0x08) != 0);

        if reg == 6 { 15 } else { 8 }
    }

    fn bit_n_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let reg = opcode & 0x07;
        let bit = (opcode >> 3) & 0x07;
        let val = self.read_reg(reg, memory);
        let result = val & (1 << bit);

        self.set_flag_z(result == 0);
        self.set_flag_n(false);
        self.set_flag_h(true);
        self.set_flag_s((bit == 7) && (result != 0));
        self.set_flag_pv(result == 0);

        if reg == 6 { 12 } else { 8 }
    }

    fn res_n_r(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let reg = opcode & 0x07;
        let bit = (opcode >> 3) & 0x07;
        let val = self.read_reg(reg, memory);
        let result = val & !(1 << bit);
        self.write_reg(reg, result, memory);

        if reg == 6 { 15 } else { 8 }
    }

    fn set_n_r(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let reg = opcode & 0x07;
        let bit = (opcode >> 3) & 0x07;
        let val = self.read_reg(reg, memory);
        let result = val | (1 << bit);
        self.write_reg(reg, result, memory);

        if reg == 6 { 15 } else { 8 }
    }
}
