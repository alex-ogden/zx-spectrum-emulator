use super::Cpu;
use crate::memory::Memory;

impl Cpu {
    pub(super) fn execute_fd_instruction(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        match opcode {
            0x09 => self.add_iy_bc(),
            0x19 => self.add_iy_de(),
            0x29 => self.add_iy_iy(),
            0x39 => self.add_iy_sp(),
            0x21 => self.ld_iy_nn(memory),
            0x23 => self.inc_iy(),
            0x2B => self.dec_iy(),
            0x34 => self.inc_iy_d(memory),
            0x35 => self.dec_iy_d(memory),
            0x36 => self.ld_iy_d_n(memory),
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x7E => self.ld_r_iy_d(opcode, memory),
            0x70 | 0x71 | 0x72 | 0x73 | 0x74 | 0x75 | 0x77 => self.ld_iy_d_r(opcode, memory),
            0x86 => self.add_a_iy_d(memory),
            0x8E => self.adc_a_iy_d(memory),
            0x96 => self.sub_iy_d(memory),
            0x9E => self.sbc_a_iy_d(memory),
            0xA6 => self.and_iy_d(memory),
            0xAE => self.xor_iy_d(memory),
            0xB6 => self.or_iy_d(memory),
            0xBE => self.cp_iy_d(memory),
            0xCB => {
                let d = self.fetch_byte(memory) as i8;
                let sub_opcode = self.fetch_byte(memory);
                self.execute_fd_cb_instruction(sub_opcode, d, memory)
            }
            0xE1 => self.pop_iy(memory),
            0xE5 => self.push_iy(memory),
            0xE9 => self.jp_iy(),
            0xF9 => self.ld_sp_iy(),
            _ => {
                eprintln!("Unknown FD opcode: 0x{:02X} at PC: 0x{:04X}", opcode, self.pc - 2);
                4
            }
        }
    }

    fn add_iy_bc(&mut self) -> u8 {
        let old_val = self.iy;
        let result = self.iy.wrapping_add(self.bc());
        self.iy = result;

        self.set_flag_c((old_val as u32 + self.bc() as u32) > 0xFFFF);
        self.set_flag_n(false);
        self.set_flag_h(((old_val & 0x0FFF) + (self.bc() & 0x0FFF)) > 0x0FFF);
        self.set_flag_x((result & 0x0800) != 0);
        self.set_flag_y((result & 0x2000) != 0);

        15
    }

    fn add_iy_de(&mut self) -> u8 {
        let old_val = self.iy;
        let result = self.iy.wrapping_add(self.de());
        self.iy = result;

        self.set_flag_c((old_val as u32 + self.de() as u32) > 0xFFFF);
        self.set_flag_n(false);
        self.set_flag_h(((old_val & 0x0FFF) + (self.de() & 0x0FFF)) > 0x0FFF);
        self.set_flag_x((result & 0x0800) != 0);
        self.set_flag_y((result & 0x2000) != 0);

        15
    }

    fn add_iy_iy(&mut self) -> u8 {
        let old_val = self.iy;
        let result = self.iy.wrapping_add(self.iy);
        self.iy = result;

        self.set_flag_c((old_val as u32 + self.iy as u32) > 0xFFFF);
        self.set_flag_n(false);
        self.set_flag_h(((old_val & 0x0FFF) + (self.iy & 0x0FFF)) > 0x0FFF);
        self.set_flag_x((result & 0x0800) != 0);
        self.set_flag_y((result & 0x2000) != 0);

        15
    }

    fn add_iy_sp(&mut self) -> u8 {
        let old_val = self.iy;
        let result = self.iy.wrapping_add(self.sp);
        self.iy = result;

        self.set_flag_c((old_val as u32 + self.sp as u32) > 0xFFFF);
        self.set_flag_n(false);
        self.set_flag_h(((old_val & 0x0FFF) + (self.sp & 0x0FFF)) > 0x0FFF);
        self.set_flag_x((result & 0x0800) != 0);
        self.set_flag_y((result & 0x2000) != 0);

        15
    }

    fn ld_iy_nn(&mut self, memory: &mut Memory) -> u8 {
        let val = self.fetch_word(memory);
        self.iy = val;
        14
    }

    fn inc_iy(&mut self) -> u8 {
        self.iy = self.iy.wrapping_add(1);
        10
    }

    fn dec_iy(&mut self) -> u8 {
        self.iy = self.iy.wrapping_sub(1);
        10
    }

    fn inc_iy_d(&mut self, memory: &mut Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
        let old_val = memory.read(addr);
        let new_val = old_val.wrapping_add(1);
        memory.write(addr, new_val);

        self.set_flag_n(false);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) == 0x0F);
        self.set_flag_pv(old_val == 0x7F);
        self.set_flag_x((new_val & 0x08) != 0);
        self.set_flag_y((new_val & 0x20) != 0);

        23
    }

    fn dec_iy_d(&mut self, memory: &mut Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
        let old_val = memory.read(addr);
        let new_val = old_val.wrapping_sub(1);
        memory.write(addr, new_val);

        self.set_flag_n(true);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) == 0x00);
        self.set_flag_pv(old_val == 0x80);
        self.set_flag_x((new_val & 0x08) != 0);
        self.set_flag_y((new_val & 0x20) != 0);

        23
    }

    fn ld_iy_d_n(&mut self, memory: &mut Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let n = self.fetch_byte(memory);
        let addr = self.iy.wrapping_add(d as u16);
        memory.write(addr, n);
        19
    }

    fn ld_r_iy_d(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);

        let reg = (opcode >> 3) & 0x07;
        match reg {
            0 => self.b = val,
            1 => self.c = val,
            2 => self.d = val,
            3 => self.e = val,
            4 => self.h = val,
            5 => self.l = val,
            7 => self.a = val,
            _ => unreachable!(),
        }

        19
    }

    fn ld_iy_d_r(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);

        let reg = opcode & 0x07;
        let val = match reg {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            7 => self.a,
            _ => unreachable!(),
        };

        memory.write(addr, val);
        19
    }

    fn add_a_iy_d(&mut self, memory: &Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let old_val = self.a;
        let new_val = self.a.wrapping_add(val);
        self.a = new_val;

        self.set_flag_c((old_val as u16 + val as u16) > 0xFF);
        self.set_flag_n(false);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) + (val & 0x0F) > 0x0F);
        self.set_flag_pv(((old_val ^ new_val) & (val ^ new_val) & 0x80) != 0);
        self.set_flag_x((new_val & 0x08) != 0);
        self.set_flag_y((new_val & 0x20) != 0);

        19
    }

    fn adc_a_iy_d(&mut self, memory: &Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let old_val = self.a;
        let carry = if self.get_flag_c() { 1 } else { 0 };
        let new_val = self.a.wrapping_add(val).wrapping_add(carry);
        self.a = new_val;

        let full_add = (old_val as u16).wrapping_add(val as u16).wrapping_add(carry as u16);

        self.set_flag_c(full_add > 0xFF);
        self.set_flag_n(false);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) + (val & 0x0F) + carry > 0x0F);
        self.set_flag_pv(((old_val ^ new_val) & (val ^ new_val) & 0x80) != 0);
        self.set_flag_x((new_val & 0x08) != 0);
        self.set_flag_y((new_val & 0x20) != 0);

        19
    }

    fn sub_iy_d(&mut self, memory: &Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let old_val = self.a;
        let new_val = old_val.wrapping_sub(val);
        self.a = new_val;

        self.set_flag_c(val > old_val);
        self.set_flag_n(true);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) < (val & 0x0F));
        self.set_flag_pv(((old_val ^ val) & (old_val ^ new_val) & 0x80) != 0);
        self.set_flag_x((new_val & 0x08) != 0);
        self.set_flag_y((new_val & 0x20) != 0);

        19
    }

    fn sbc_a_iy_d(&mut self, memory: &Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let old_val = self.a;
        let carry = if self.get_flag_c() { 1 } else { 0 };
        let new_val = self.a.wrapping_sub(val).wrapping_sub(carry);
        self.a = new_val;

        let full_sub = (old_val as u16).wrapping_sub(val as u16).wrapping_sub(carry as u16);

        self.set_flag_c(full_sub > 0xFF);
        self.set_flag_n(true);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) < (val & 0x0F) + carry);
        self.set_flag_pv(((old_val ^ val) & (old_val ^ new_val) & 0x80) != 0);
        self.set_flag_x((new_val & 0x08) != 0);
        self.set_flag_y((new_val & 0x20) != 0);

        19
    }

    fn and_iy_d(&mut self, memory: &Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        self.a &= val;

        self.set_flag_c(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_h(true);
        self.set_flag_z(self.a == 0);
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_x((self.a & 0x08) != 0);
        self.set_flag_y((self.a & 0x20) != 0);

        19
    }

    fn xor_iy_d(&mut self, memory: &Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        self.a ^= val;

        self.set_flag_c(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_h(false);
        self.set_flag_z(self.a == 0);
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_x((self.a & 0x08) != 0);
        self.set_flag_y((self.a & 0x20) != 0);

        19
    }

    fn or_iy_d(&mut self, memory: &Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        self.a |= val;

        self.set_flag_c(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_h(false);
        self.set_flag_z(self.a == 0);
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_x((self.a & 0x08) != 0);
        self.set_flag_y((self.a & 0x20) != 0);

        19
    }

    fn cp_iy_d(&mut self, memory: &Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let result = self.a.wrapping_sub(val);

        self.set_flag_c(val > self.a);
        self.set_flag_n(true);
        self.set_flag_pv(((self.a ^ val) & (self.a ^ result) & 0x80) != 0);
        self.set_flag_h((self.a & 0x0F) < (val & 0x0F));
        self.set_flag_z(self.a == val);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_x((result & 0x20) != 0);
        self.set_flag_y((result & 0x08) != 0);

        19
    }

    fn execute_fd_cb_instruction(&mut self, opcode: u8, d: i8, memory: &mut Memory) -> u8 {
        match opcode {
            0x06 => self.rlc_iy_d(d, memory),
            0x0E => self.rrc_iy_d(d, memory),
            0x16 => self.rl_iy_d(d, memory),
            0x1E => self.rr_iy_d(d, memory),
            0x26 => self.sla_iy_d(d, memory),
            0x2E => self.sra_iy_d(d, memory),
            0x3E => self.srl_iy_d(d, memory),
            0x46..=0x7E => self.bit_n_iy_d(opcode, d, memory),
            0x86..=0xBE => self.res_n_iy_d(opcode, d, memory),
            0xC6..=0xFE => self.set_n_iy_d(opcode, d, memory),
            _ => {
                eprintln!("Unknown FD CB opcode: 0x{:02X} at PC: 0x{:04X}", opcode, self.pc - 4);
                23
            }
        }
    }

    fn rlc_iy_d(&mut self, d: i8, memory: &mut Memory) -> u8 {
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let bit7 = val >> 7;
        let result = (val << 1) | bit7;
        memory.write(addr, result);

        self.set_flag_c(bit7 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_pv(result.count_ones() % 2 == 0);
        self.set_flag_x((result & 0x08) != 0);
        self.set_flag_y((result & 0x20) != 0);

        23
    }

    fn rrc_iy_d(&mut self, d: i8, memory: &mut Memory) -> u8 {
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let bit0 = val & 1;
        let result = (val >> 1) | (bit0 << 7);
        memory.write(addr, result);

        self.set_flag_c(bit0 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_pv(result.count_ones() % 2 == 0);
        self.set_flag_x((result & 0x08) != 0);
        self.set_flag_y((result & 0x20) != 0);

        23
    }

    fn rl_iy_d(&mut self, d: i8, memory: &mut Memory) -> u8 {
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let old_carry = if self.get_flag_c() { 1 } else { 0 };
        let bit7 = val >> 7;
        let result = (val << 1) | old_carry;
        memory.write(addr, result);

        self.set_flag_c(bit7 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_pv(result.count_ones() % 2 == 0);
        self.set_flag_x((result & 0x08) != 0);
        self.set_flag_y((result & 0x20) != 0);

        23
    }

    fn rr_iy_d(&mut self, d: i8, memory: &mut Memory) -> u8 {
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let old_carry = if self.get_flag_c() { 0x80 } else { 0 };
        let bit0 = val & 1;
        let result = (val >> 1) | old_carry;
        memory.write(addr, result);

        self.set_flag_c(bit0 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_pv(result.count_ones() % 2 == 0);
        self.set_flag_x((result & 0x08) != 0);
        self.set_flag_y((result & 0x20) != 0);

        23
    }

    fn sla_iy_d(&mut self, d: i8, memory: &mut Memory) -> u8 {
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let bit7 = val >> 7;
        let result = val << 1;
        memory.write(addr, result);

        self.set_flag_c(bit7 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_pv(result.count_ones() % 2 == 0);
        self.set_flag_x((result & 0x08) != 0);
        self.set_flag_y((result & 0x20) != 0);

        23
    }

    fn sra_iy_d(&mut self, d: i8, memory: &mut Memory) -> u8 {
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let bit7 = val & 0x80;
        let bit0 = val & 1;
        let result = (val >> 1) | bit7;
        memory.write(addr, result);

        self.set_flag_c(bit0 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_pv(result.count_ones() % 2 == 0);
        self.set_flag_x((result & 0x08) != 0);
        self.set_flag_y((result & 0x20) != 0);

        23
    }

    fn srl_iy_d(&mut self, d: i8, memory: &mut Memory) -> u8 {
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let bit0 = val & 1;
        let result = val >> 1;
        memory.write(addr, result);

        self.set_flag_c(bit0 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_z(result == 0);
        self.set_flag_s(false);
        self.set_flag_pv(result.count_ones() % 2 == 0);
        self.set_flag_x((result & 0x08) != 0);
        self.set_flag_y((result & 0x20) != 0);

        23
    }

    fn bit_n_iy_d(&mut self, opcode: u8, d: i8, memory: &Memory) -> u8 {
        let bit = (opcode >> 3) & 0x07;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let result = val & (1 << bit);

        self.set_flag_z(result == 0);
        self.set_flag_n(false);
        self.set_flag_h(true);
        self.set_flag_s((bit == 7) && (result != 0));
        self.set_flag_pv(result == 0);

        20
    }

    fn res_n_iy_d(&mut self, opcode: u8, d: i8, memory: &mut Memory) -> u8 {
        let bit = (opcode >> 3) & 0x07;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let result = val & !(1 << bit);
        memory.write(addr, result);

        23
    }

    fn set_n_iy_d(&mut self, opcode: u8, d: i8, memory: &mut Memory) -> u8 {
        let bit = (opcode >> 3) & 0x07;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let result = val | (1 << bit);
        memory.write(addr, result);

        23
    }

    fn pop_iy(&mut self, memory: &Memory) -> u8 {
        self.iy = self.pop(memory);
        14
    }

    fn push_iy(&mut self, memory: &mut Memory) -> u8 {
        self.push(self.iy, memory);
        15
    }

    fn jp_iy(&mut self) -> u8 {
        self.pc = self.iy;
        8
    }

    fn ld_sp_iy(&mut self) -> u8 {
        self.sp = self.iy;
        10
    }
}
