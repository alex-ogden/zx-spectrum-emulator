use super::Cpu;
use crate::memory::Memory;

impl Cpu {
    pub(super) fn execute_instruction(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        match opcode {
            // ED-prefixed instructions
            0xED => {
                let sub_opcode = self.fetch_byte(memory);
                self.execute_ed_instruction(sub_opcode, memory)
            }
            // CB-prefixed instructions
            0xCB => {
                let sub_opcode = self.fetch_byte(memory);
                self.execute_cb_instruction(sub_opcode, memory)
            }
            // DD-prefixed instructions
            0xDD => {
                let sub_opcode = self.fetch_byte(memory);
                self.execute_dd_instruction(sub_opcode, memory)
            }
            // FD-prefixed instructions
            0xFD => {
                let sub_opcode = self.fetch_byte(memory);
                self.execute_fd_instruction(sub_opcode, memory)
            }

            // Regular non-prefixed instructions
            0x00 => self.nop(),
            0x76 => self.halt(),
            0x17 => self.rla(),
            0x1F => self.rra(),
            0x07 => self.rlca(),
            0x0F => self.rrca(),
            0x27 => self.daa(),
            0x2F => self.cpl(),
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => self.ld_r_n(opcode, memory),
            0x01 | 0x11 | 0x21 | 0x31 => self.ld_rr_nn(opcode, memory),
            0x40..=0x7F => self.ld_r_r(opcode, memory),
            0x02 | 0x12 => self.ld_rr_indirect_a(opcode, memory),
            0x0A | 0x1A => self.ld_a_rr_indirect(opcode, memory),
            0x32 => self.ld_nn_indirect_a(memory),
            0x3A => self.ld_a_nn_indirect(memory),
            0x22 => self.ld_nn_indirect_hl(memory),
            0x2A => self.ld_hl_nn_indirect(memory),
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x3C => self.inc_r(opcode),
            0x03 | 0x13 | 0x23 | 0x33 => self.inc_rr(opcode),
            0x34 => self.inc_hl_indirect(memory),
            0x05 | 0x15 | 0x25 | 0x0D | 0x1D | 0x2D | 0x3D => self.dec_r(opcode),
            0x0B | 0x1B | 0x2B | 0x3B => self.dec_rr(opcode),
            0x35 => self.dec_hl_indirect(memory),
            0x10 => self.dec_jnz_d(memory),
            0x80..=0x87 => self.add_a_r(opcode, memory),
            0x90..=0x97 => self.sub_a_r(opcode, memory),
            0x88..=0x8F => self.adc_a_r(opcode, memory),
            0x98..=0x9F => self.sbc_a_r(opcode, memory),
            0xC6 => self.add_a_n(memory),
            0xCE => self.adc_a_n(memory),
            0xD6 => self.sub_n(memory),
            0xDE => self.sbc_a_n(memory),
            0xE6 => self.and_n(memory),
            0xEE => self.xor_n(memory),
            0xF6 => self.or_n(memory),
            0xFE => self.cp_n(memory),
            0x37 => self.scf(),
            0x3F => self.ccf(),
            0xC3 => self.jp_nn(memory),
            0xE9 => self.jp_hl(),
            0xC2 | 0xCA | 0xD2 | 0xDA | 0xE2 | 0xEA | 0xF2 | 0xFA => self.jp_cc_nn(opcode, memory),
            0x18 | 0x20 | 0x28 | 0x30 | 0x38 => self.jr_cc_e(opcode, memory),
            0xCD | 0xC4 | 0xCC | 0xD4 | 0xDC | 0xE4 | 0xEC | 0xF4 | 0xFC => {
                self.call_cc_nn(opcode, memory)
            }
            0xC9 | 0xC0 | 0xC8 | 0xD0 | 0xD8 | 0xE0 | 0xE8 | 0xF0 | 0xF8 => {
                self.ret_cc(opcode, memory)
            }
            0xC5 | 0xD5 | 0xE5 | 0xF5 => self.push_rr(opcode, memory),
            0xC1 | 0xD1 | 0xE1 | 0xF1 => self.pop_rr(opcode, memory),
            0xA0..=0xA7 => self.and_a_r(opcode, memory),
            0xB0..=0xB7 => self.or_a_r(opcode, memory),
            0xA8..=0xAF => self.xor_a_r(opcode, memory),
            0xB8..=0xBF => self.cp_a_r(opcode, memory),
            0xF3 => self.di(),
            0xFB => self.ei(),
            0xD3 => self.out_n_a(memory),
            0xDB => self.in_a_n(memory),
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => self.rst_nn(opcode, memory),
            0x09 | 0x19 | 0x29 | 0x39 => self.add_hl_rr(opcode),
            0xEB => self.ex_de_hl(),
            0x08 => self.ex_af_af_prime(),
            0xD9 => self.exx(),
            0xE3 => self.ex_sp_hl(memory),
            0xF9 => self.ld_sp_hl(),
        }
    }
    fn nop(&mut self) -> u8 {
        4
    }

    fn halt(&mut self) -> u8 {
        self.is_halted = true;
        4
    }

    fn di(&mut self) -> u8 {
        self.iff1 = false;
        self.iff2 = false;
        4
    }

    fn ei(&mut self) -> u8 {
        self.iff1 = true;
        self.iff2 = true;
        4
    }

    // Stub I/O operations (to be implemented with I/O controller later)
    fn out_n_a(&mut self, memory: &Memory) -> u8 {
        let _port = self.fetch_byte(memory);
        // TODO: Implement port output when I/O controller is added
        11
    }

    fn in_a_n(&mut self, memory: &Memory) -> u8 {
        let _port = self.fetch_byte(memory);
        // TODO: Implement port input when I/O controller is added
        self.a = 0xFF; // Default to reading 0xFF (floating bus)
        11
    }

    fn rla(&mut self) -> u8 {
        let bit7 = self.a >> 7;
        let carry_bit = if self.get_flag_c() { 1 } else { 0 };
        let result = (self.a << 1) | carry_bit;
        self.a = result;

        self.set_flag_c(bit7 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_y((result & 0x20) != 0);
        self.set_flag_x((result & 0x08) != 0);

        4
    }
    fn rlca(&mut self) -> u8 {
        let bit7 = self.a >> 7;
        let result = (self.a << 1) | bit7;
        self.a = result;

        self.set_flag_c(bit7 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_y((result & 0x20) != 0);
        self.set_flag_x((result & 0x08) != 0);

        4
    }
    fn rra(&mut self) -> u8 {
        let bit0 = self.a & 1;
        let carry_bit = if self.get_flag_c() { 0x80 } else { 0 };
        let result = (self.a >> 1) | carry_bit;
        self.a = result;

        self.set_flag_c(bit0 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_y((result & 0x20) != 0);
        self.set_flag_x((result & 0x08) != 0);

        4
    }
    fn rrca(&mut self) -> u8 {
        let bit0 = self.a & 1;
        let result = (self.a >> 1) | (bit0 << 7);
        self.a = result;

        self.set_flag_c(bit0 == 1);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_y((result & 0x20) != 0);
        self.set_flag_x((result & 0x08) != 0);

        4
    }

    fn daa(&mut self) -> u8 {
        // Decimal Adjust Accumulator for BCD arithmetic
        let mut correction = 0u8;
        let mut carry = false;

        // Check if we need to adjust the lower nibble
        if self.get_flag_h() || (self.a & 0x0F) > 9 {
            correction |= 0x06;
        }

        // Check if we need to adjust the upper nibble
        if self.get_flag_c() || self.a > 0x99 {
            correction |= 0x60;
            carry = true;
        }

        // Apply correction based on whether last operation was addition or subtraction
        if self.get_flag_n() {
            // Subtraction - adjust downward
            self.a = self.a.wrapping_sub(correction);
        } else {
            // Addition - adjust upward
            self.a = self.a.wrapping_add(correction);
        }

        // Set flags
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_z(self.a == 0);
        self.set_flag_h((correction & 0x06) != 0);
        // DAA does not affect PV flag
        self.set_flag_c(carry);
        self.set_flag_y((self.a & 0x20) != 0);
        self.set_flag_x((self.a & 0x08) != 0);

        4
    }

    fn cpl(&mut self) -> u8 {
        self.a = !self.a;
        self.set_flag_n(true);
        self.set_flag_h(true);
        self.set_flag_y((self.a & 0x20) != 0);
        self.set_flag_x((self.a & 0x08) != 0);
        4
    }
    fn ex_de_hl(&mut self) -> u8 {
        let temp = self.de();
        self.set_de(self.hl());
        self.set_hl(temp);
        4
    }
    fn ex_af_af_prime(&mut self) -> u8 {
        let temp = self.af();
        self.set_af(self.af_shadow);
        self.af_shadow = temp;
        4
    }
    fn exx(&mut self) -> u8 {
        let temp_bc = self.bc();
        let temp_de = self.de();
        let temp_hl = self.hl();

        self.set_bc(self.bc_shadow);
        self.set_de(self.de_shadow);
        self.set_hl(self.hl_shadow);

        self.bc_shadow = temp_bc;
        self.de_shadow = temp_de;
        self.hl_shadow = temp_hl;

        4
    }
    fn ex_sp_hl(&mut self, memory: &mut Memory) -> u8 {
        let temp_sp = memory.read_word(self.sp);
        let temp_hl = self.hl();

        memory.write_word(self.sp, temp_hl);
        self.set_hl(temp_sp);

        19
    }
    fn jp_hl(&mut self) -> u8 {
        self.pc = self.hl();
        4
    }
    fn add_hl_rr(&mut self, opcode: u8) -> u8 {
        let src_reg = match opcode {
            0x09 => self.bc(),
            0x19 => self.de(),
            0x29 => self.hl(),
            0x39 => self.sp,
            _ => unreachable!("Invalid ADD HL, rr opcode: 0x{:02X}", opcode),
        };

        let old_val = self.hl();
        let result = self.hl().wrapping_add(src_reg);
        let intermediate_res = (old_val as u32).wrapping_add(src_reg as u32);
        self.set_hl(result);

        self.set_flag_c(intermediate_res > 0xFFFF);
        self.set_flag_n(false);
        self.set_flag_h(((old_val & 0x0FFF) + (src_reg & 0x0FFF)) > 0x0FFF);
        self.set_flag_y((self.h & 0x20) != 0);
        self.set_flag_x((self.h & 0x08) != 0);

        11
    }
    fn rst_nn(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let addr = (opcode & 0x38) as u16;
        self.push(self.pc, memory);
        self.pc = addr;
        11
    }
    fn ld_rr_indirect_a(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let addr = match opcode {
            0x02 => self.bc(),
            0x12 => self.de(),
            _ => unreachable!(),
        };
        memory.write(addr, self.a);
        7
    }
    fn ld_a_rr_indirect(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let addr = match opcode {
            0x0A => self.bc(),
            0x1A => self.de(),
            _ => unreachable!(),
        };
        self.a = memory.read(addr);
        7
    }
    fn ld_nn_indirect_a(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.fetch_word(memory);
        memory.write(addr, self.a);
        13
    }
    fn ld_a_nn_indirect(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.fetch_word(memory);
        self.a = memory.read(addr);
        13
    }
    fn ld_nn_indirect_hl(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.fetch_word(memory);
        memory.write_word(addr, self.hl());
        16
    }
    fn ld_hl_nn_indirect(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.fetch_word(memory);
        let val = memory.read_word(addr);
        self.set_hl(val);
        16
    }
    fn and_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let src = self.read_reg(src_code, memory);
        self.a &= src;

        self.set_flag_c(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_h(true);
        self.set_flag_z(self.a == 0);
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_y((self.a & 0x20) != 0);
        self.set_flag_x((self.a & 0x08) != 0);

        if src_code == 6 {
            7
        } else {
            4
        }
    }

    fn or_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let src = self.read_reg(src_code, memory);
        self.a |= src;

        self.set_flag_c(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_h(false);
        self.set_flag_z(self.a == 0);
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_y((self.a & 0x20) != 0);
        self.set_flag_x((self.a & 0x08) != 0);

        if src_code == 6 {
            7
        } else {
            4
        }
    }

    fn xor_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let src = self.read_reg(src_code, memory);
        self.a ^= src;

        self.set_flag_c(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_h(false);
        self.set_flag_z(self.a == 0);
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_y((self.a & 0x20) != 0);
        self.set_flag_x((self.a & 0x08) != 0);

        if src_code == 6 {
            7
        } else {
            4
        }
    }

    fn cp_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let src = self.read_reg(src_code, memory);
        let result = self.a.wrapping_sub(src);

        self.set_flag_c(src > self.a);
        self.set_flag_n(true);
        self.set_flag_pv(((self.a ^ src) & (self.a ^ result) & 0x80) != 0);
        self.set_flag_h((self.a & 0x0F) < (src & 0x0F));
        self.set_flag_z(self.a == src);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_y((result & 0x20) != 0);
        self.set_flag_x((result & 0x08) != 0);

        if src_code == 6 {
            7
        } else {
            4
        }
    }
    fn push_rr(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let val = match opcode {
            0xC5 => self.bc(),
            0xD5 => self.de(),
            0xE5 => self.hl(),
            0xF5 => self.af(),
            _ => unreachable!("Invalid PUSH rr opcode: 0x{:02X}", opcode),
        };

        self.push(val, memory);
        11
    }
    fn pop_rr(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let val = self.pop(memory);
        match opcode {
            0xC1 => self.set_bc(val),
            0xD1 => self.set_de(val),
            0xE1 => self.set_hl(val),
            0xF1 => self.set_af(val),
            _ => unreachable!("Invalid POP rr opcode: 0x{:02X}", opcode),
        }

        10
    }
    fn call_cc_nn(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let addr = self.fetch_word(memory);

        let (condition, cycles_taken, cycles_not_taken) = match opcode {
            0xCD => (true, 17, 17),
            0xC4 => (!self.get_flag_z(), 17, 10),
            0xCC => (self.get_flag_z(), 17, 10),
            0xD4 => (!self.get_flag_c(), 17, 10),
            0xDC => (self.get_flag_c(), 17, 10),
            0xE4 => (!self.get_flag_pv(), 17, 10),
            0xEC => (self.get_flag_pv(), 17, 10),
            0xF4 => (!self.get_flag_s(), 17, 10),
            0xFC => (self.get_flag_s(), 17, 10),
            _ => unreachable!("Invalid CALL e opcode: 0x{:02X}", opcode),
        };

        if condition {
            self.push(self.pc, memory);
            self.pc = addr;
            cycles_taken
        } else {
            cycles_not_taken
        }
    }
    fn ret_cc(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let (condition, cycles_taken, cycles_not_taken) = match opcode {
            0xC9 => (true, 10, 10),
            0xC0 => (!self.get_flag_z(), 11, 5),
            0xC8 => (self.get_flag_z(), 11, 5),
            0xD0 => (!self.get_flag_c(), 11, 5),
            0xD8 => (self.get_flag_c(), 11, 5),
            0xE0 => (!self.get_flag_pv(), 11, 5),
            0xE8 => (self.get_flag_pv(), 11, 5),
            0xF0 => (!self.get_flag_s(), 11, 5),
            0xF8 => (self.get_flag_s(), 11, 5),
            _ => unreachable!("Invalid RET cc opcode: 0x{:02X}", opcode),
        };

        if condition {
            self.pc = self.pop(memory);
            cycles_taken
        } else {
            cycles_not_taken
        }
    }
    fn ld_r_n(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let val = self.fetch_byte(memory);
        let reg = (opcode >> 3) & 0x07;
        self.write_reg(reg, val, memory);

        if reg == 6 {
            10
        } else {
            7
        }
    }
    fn ld_rr_nn(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let val = self.fetch_word(memory);
        match (opcode >> 4) & 0x03 {
            0 => self.set_bc(val),
            1 => self.set_de(val),
            2 => self.set_hl(val),
            3 => self.sp = val,
            _ => panic!("Invalid LD rr nn opcode: 0x{:02X}", opcode),
        }
        10
    }
    fn ld_r_r(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let src_code = opcode & 0x07;
        let dest_code = (opcode >> 3) & 0x07;
        let val = self.read_reg(src_code, memory);
        self.write_reg(dest_code, val, memory);

        if src_code == 6 || dest_code == 6 {
            // Memory operations take 7 cycles
            7
        } else {
            // Reg -> Reg operations take 4 cycles
            4
        }
    }
    fn inc_r(&mut self, opcode: u8) -> u8 {
        // Logic is the same on all INC operations
        let reg = match opcode {
            0x04 => &mut self.b,
            0x0C => &mut self.c,
            0x14 => &mut self.d,
            0x1C => &mut self.e,
            0x24 => &mut self.h,
            0x2C => &mut self.l,
            0x3C => &mut self.a,
            _ => panic!("Invalid INC r opcode: 0x{:02X}", opcode),
        };

        let old_val = *reg;
        *reg = old_val.wrapping_add(1);
        let new_val = *reg;

        self.set_flag_n(false);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) == 0x0F);
        self.set_flag_pv(old_val == 0x7F);
        self.set_flag_y((new_val & 0x20) != 0);
        self.set_flag_x((new_val & 0x08) != 0);

        4
    }
    fn inc_rr(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x03 => self.set_bc(self.bc().wrapping_add(1)),
            0x13 => self.set_de(self.de().wrapping_add(1)),
            0x23 => self.set_hl(self.hl().wrapping_add(1)),
            0x33 => self.sp = self.sp.wrapping_add(1),
            _ => unreachable!("Invalid INC rr opcode: 0x{:02X}", opcode),
        }
        6
    }
    fn inc_hl_indirect(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.hl();
        let old_val = memory.read(addr);
        let new_val = old_val.wrapping_add(1);
        memory.write(addr, new_val);

        self.set_flag_n(false);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) == 0x0F);
        self.set_flag_pv(old_val == 0x7F);
        self.set_flag_y((new_val & 0x20) != 0);
        self.set_flag_x((new_val & 0x08) != 0);

        11
    }
    fn dec_r(&mut self, opcode: u8) -> u8 {
        let reg = match opcode {
            0x05 => &mut self.b,
            0x0D => &mut self.c,
            0x15 => &mut self.d,
            0x1D => &mut self.e,
            0x25 => &mut self.h,
            0x2D => &mut self.l,
            0x3D => &mut self.a,
            _ => panic!("Invalid DEC r opcode: 0x{:02X}", opcode),
        };

        let old_val = *reg;
        *reg = old_val.wrapping_sub(1);
        let new_val = *reg;

        self.set_flag_n(true);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) == 0x00);
        self.set_flag_pv(old_val == 0x80);
        self.set_flag_y((new_val & 0x20) != 0);
        self.set_flag_x((new_val & 0x08) != 0);

        4
    }
    fn dec_rr(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x0B => self.set_bc(self.bc().wrapping_sub(1)),
            0x1B => self.set_de(self.de().wrapping_sub(1)),
            0x2B => self.set_hl(self.hl().wrapping_sub(1)),
            0x3B => self.sp = self.sp.wrapping_sub(1),
            _ => unreachable!("Invalid DEC rr opcode: 0x{:02X}", opcode),
        }
        6
    }
    fn dec_hl_indirect(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.hl();
        let old_val = memory.read(addr);
        let new_val = old_val.wrapping_sub(1);
        memory.write(addr, new_val);

        self.set_flag_n(true);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) == 0x00);
        self.set_flag_pv(old_val == 0x80);
        self.set_flag_y((new_val & 0x20) != 0);
        self.set_flag_x((new_val & 0x08) != 0);

        11
    }
    fn dec_jnz_d(&mut self, memory: &Memory) -> u8 {
        let offset = self.fetch_byte(memory) as i8;
        self.b = self.b.wrapping_sub(1);
        if self.b != 0 {
            self.pc = self.pc.wrapping_add(offset as i16 as u16);
            return 13;
        }
        8
    }
    fn jp_nn(&mut self, memory: &Memory) -> u8 {
        let addr = self.fetch_word(memory);
        self.pc = addr;
        10
    }
    fn jp_cc_nn(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let addr = self.fetch_word(memory);

        let condition = match opcode {
            0xC2 => !self.get_flag_z(),
            0xCA => self.get_flag_z(),
            0xD2 => !self.get_flag_c(),
            0xDA => self.get_flag_c(),
            0xE2 => !self.get_flag_pv(),
            0xEA => self.get_flag_pv(),
            0xF2 => !self.get_flag_s(),
            0xFA => self.get_flag_s(),
            _ => unreachable!("Invalid JP cc opcode: 0x{:02X}", opcode),
        };

        // If selected condition is true, jump
        if condition {
            self.pc = addr;
        }
        10
    }
    fn jr_cc_e(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let offset = self.fetch_byte(memory) as i8;

        let (condition, cycles_taken, cycles_not_taken) = match opcode {
            0x18 => (true, 12, 12),
            0x20 => (!self.get_flag_z(), 12, 7),
            0x28 => (self.get_flag_z(), 12, 7),
            0x30 => (!self.get_flag_c(), 12, 7),
            0x38 => (self.get_flag_c(), 12, 7),
            _ => unreachable!("Invalid JR cc opcode: 0x{:02X}", opcode),
        };

        if condition {
            self.pc = self.pc.wrapping_add(offset as i16 as u16);
            cycles_taken
        } else {
            cycles_not_taken
        }
    }
    fn add_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let old_val = self.a;
        let val = self.read_reg(src_code, memory);
        let new_val = self.a.wrapping_add(val);
        self.a = new_val;

        self.set_flag_c((old_val as u16) + (val as u16) > 0xFF);
        self.set_flag_n(false);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) + (val & 0x0F) > 0x0F);
        self.set_flag_pv(((old_val ^ new_val) & (val ^ new_val) & 0x80) != 0);
        self.set_flag_y((new_val & 0x20) != 0);
        self.set_flag_x((new_val & 0x08) != 0);

        // Reg->Reg takes 4 cycles
        // Memory->Reg takes 7 cycles
        if src_code == 6 {
            7
        } else {
            4
        }
    }
    fn adc_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let old_val = self.a;
        let val = self.read_reg(src_code, memory);
        let carry = if self.get_flag_c() { 1u8 } else { 0u8 };
        let new_val = self.a.wrapping_add(val).wrapping_add(carry);
        self.a = new_val;

        let full_add = (old_val as u16)
            .wrapping_add(val as u16)
            .wrapping_add(carry as u16);

        self.set_flag_c(full_add > 0xFF);
        self.set_flag_n(false);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) + (val & 0x0F) + carry > 0x0F);
        self.set_flag_pv(((old_val ^ new_val) & (val ^ new_val) & 0x80) != 0);
        self.set_flag_y((new_val & 0x20) != 0);
        self.set_flag_x((new_val & 0x08) != 0);

        if src_code == 6 {
            7
        } else {
            4
        }
    }
    fn sub_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let old_val = self.a;
        let val = self.read_reg(src_code, memory);
        let new_val = old_val.wrapping_sub(val);
        self.a = new_val;

        self.set_flag_c(val > old_val);
        self.set_flag_n(true);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) < (val & 0x0F));
        self.set_flag_pv(((old_val ^ val) & (old_val ^ new_val) & 0x80) != 0);
        self.set_flag_y((new_val & 0x20) != 0);
        self.set_flag_x((new_val & 0x08) != 0);

        if src_code == 6 {
            7
        } else {
            4
        }
    }
    fn sbc_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let old_val = self.a;
        let val = self.read_reg(src_code, memory);
        let carry = if self.get_flag_c() { 1u8 } else { 0u8 };
        let new_val = self.a.wrapping_sub(val).wrapping_sub(carry);
        self.a = new_val;

        let full_sub = (old_val as u16)
            .wrapping_sub(val as u16)
            .wrapping_sub(carry as u16);

        self.set_flag_c(full_sub > 0xFF);
        self.set_flag_n(true);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) < (val & 0x0F) + carry);
        self.set_flag_pv(((old_val ^ val) & (old_val ^ new_val) & 0x80) != 0);
        self.set_flag_y((new_val & 0x20) != 0);
        self.set_flag_x((new_val & 0x08) != 0);

        if src_code == 6 {
            7
        } else {
            4
        }
    }
    fn scf(&mut self) -> u8 {
        self.set_flag_c(true);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_y((self.a & 0x20) != 0);
        self.set_flag_x((self.a & 0x08) != 0);
        4
    }
    fn ccf(&mut self) -> u8 {
        let old_carry = self.get_flag_c();
        self.set_flag_h(old_carry);
        self.set_flag_c(!old_carry);
        self.set_flag_n(false);
        self.set_flag_y((self.a & 0x20) != 0);
        self.set_flag_x((self.a & 0x08) != 0);
        4
    }
    fn add_a_n(&mut self, memory: &Memory) -> u8 {
        let val = self.fetch_byte(memory);
        let old_val = self.a;
        let new_val = self.a.wrapping_add(val);
        self.a = new_val;

        self.set_flag_c((old_val as u16) + (val as u16) > 0xFF);
        self.set_flag_n(false);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) + (val & 0x0F) > 0x0F);
        self.set_flag_pv(((old_val ^ new_val) & (val ^ new_val) & 0x80) != 0);
        self.set_flag_y((new_val & 0x20) != 0);
        self.set_flag_x((new_val & 0x08) != 0);

        7
    }
    fn adc_a_n(&mut self, memory: &Memory) -> u8 {
        let val = self.fetch_byte(memory);
        let old_val = self.a;
        let carry = if self.get_flag_c() { 1u8 } else { 0u8 };
        let new_val = self.a.wrapping_add(val).wrapping_add(carry);
        self.a = new_val;

        let full_add = (old_val as u16)
            .wrapping_add(val as u16)
            .wrapping_add(carry as u16);

        self.set_flag_c(full_add > 0xFF);
        self.set_flag_n(false);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) + (val & 0x0F) + carry > 0x0F);
        self.set_flag_pv(((old_val ^ new_val) & (val ^ new_val) & 0x80) != 0);
        self.set_flag_y((new_val & 0x20) != 0);
        self.set_flag_x((new_val & 0x08) != 0);

        7
    }
    fn sub_n(&mut self, memory: &Memory) -> u8 {
        let val = self.fetch_byte(memory);
        let old_val = self.a;
        let new_val = old_val.wrapping_sub(val);
        self.a = new_val;

        self.set_flag_c(val > old_val);
        self.set_flag_n(true);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) < (val & 0x0F));
        self.set_flag_pv(((old_val ^ val) & (old_val ^ new_val) & 0x80) != 0);
        self.set_flag_y((new_val & 0x20) != 0);
        self.set_flag_x((new_val & 0x08) != 0);

        7
    }
    fn sbc_a_n(&mut self, memory: &Memory) -> u8 {
        let val = self.fetch_byte(memory);
        let old_val = self.a;
        let carry = if self.get_flag_c() { 1u8 } else { 0u8 };
        let new_val = self.a.wrapping_sub(val).wrapping_sub(carry);
        self.a = new_val;

        let full_sub = (old_val as u16)
            .wrapping_sub(val as u16)
            .wrapping_sub(carry as u16);

        self.set_flag_c(full_sub > 0xFF);
        self.set_flag_n(true);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) < (val & 0x0F) + carry);
        self.set_flag_pv(((old_val ^ val) & (old_val ^ new_val) & 0x80) != 0);
        self.set_flag_y((new_val & 0x20) != 0);
        self.set_flag_x((new_val & 0x08) != 0);

        7
    }
    fn and_n(&mut self, memory: &Memory) -> u8 {
        let val = self.fetch_byte(memory);
        self.a &= val;

        self.set_flag_c(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_h(true);
        self.set_flag_z(self.a == 0);
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_y((self.a & 0x20) != 0);
        self.set_flag_x((self.a & 0x08) != 0);

        7
    }
    fn xor_n(&mut self, memory: &Memory) -> u8 {
        let val = self.fetch_byte(memory);
        self.a ^= val;

        self.set_flag_c(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_h(false);
        self.set_flag_z(self.a == 0);
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_y((self.a & 0x20) != 0);
        self.set_flag_x((self.a & 0x08) != 0);

        7
    }
    fn or_n(&mut self, memory: &Memory) -> u8 {
        let val = self.fetch_byte(memory);
        self.a |= val;

        self.set_flag_c(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_h(false);
        self.set_flag_z(self.a == 0);
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_y((self.a & 0x20) != 0);
        self.set_flag_x((self.a & 0x08) != 0);

        7
    }
    fn cp_n(&mut self, memory: &Memory) -> u8 {
        let val = self.fetch_byte(memory);
        let result = self.a.wrapping_sub(val);

        self.set_flag_c(val > self.a);
        self.set_flag_n(true);
        self.set_flag_pv(((self.a ^ val) & (self.a ^ result) & 0x80) != 0);
        self.set_flag_h((self.a & 0x0F) < (val & 0x0F));
        self.set_flag_z(self.a == val);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_y((result & 0x20) != 0);
        self.set_flag_x((result & 0x08) != 0);

        7
    }

    fn ld_sp_hl(&mut self) -> u8 {
        self.sp = self.hl();
        6
    }
}
