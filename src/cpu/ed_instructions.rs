use super::Cpu;
use crate::memory::Memory;

impl Cpu {
    pub(super) fn execute_ed_instruction(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        match opcode {
            0x4F => self.ld_r_a(),
            0x47 => self.ld_i_a(),
            0x5F => self.ld_a_r(),
            0x57 => self.ld_a_i(),
            0x56 | 0x76 => self.im_1(),
            0x5E => self.im_2(),
            0x46 => self.im_0(),
            0x44 | 0x4C | 0x54 | 0x5C | 0x64 | 0x6C | 0x74 | 0x7C => self.neg(),

            // Block operations
            0xA0 => self.ldi(memory),
            0xA1 => self.cpi(memory),
            0xA2 => self.ini(memory),
            0xA3 => self.outi(memory),
            0xA8 => self.ldd(memory),
            0xA9 => self.cpd(memory),
            0xAA => self.ind(memory),
            0xAB => self.outd(memory),
            0xB0 => self.ldir(memory),
            0xB1 => self.cpir(memory),
            0xB2 => self.inir(memory),
            0xB3 => self.otir(memory),
            0xB8 => self.lddr(memory),
            0xB9 => self.cpdr(memory),
            0xBA => self.indr(memory),
            0xBB => self.otdr(memory),

            // 16-bit operations
            0x4B | 0x5B | 0x6B | 0x7B => self.ld_rr_nn_indirect(opcode, memory),
            0x43 | 0x53 | 0x63 | 0x73 => self.ld_nn_indirect_rr(opcode, memory),
            0x42 | 0x52 | 0x62 | 0x72 => self.sbc_hl_rr(opcode),
            0x4A | 0x5A | 0x6A | 0x7A => self.adc_hl_rr(opcode),

            // I/O operations - for now just return dummy values
            0x40 | 0x48 | 0x50 | 0x58 | 0x60 | 0x68 | 0x78 => self.in_r_c(opcode),
            0x41 | 0x49 | 0x51 | 0x59 | 0x61 | 0x69 | 0x79 => self.out_c_r(opcode),

            // RRD and RLD
            0x67 => self.rrd(memory),
            0x6F => self.rld(memory),

            // RETN and RETI
            0x45 | 0x55 | 0x5D | 0x65 | 0x6D | 0x75 | 0x7D => self.retn(memory),
            0x4D => self.reti(memory),

            _ => {
                eprintln!(
                    "Unknown ED opcode: 0x{:02X} at PC: 0x{:04X}",
                    opcode,
                    self.pc - 2
                );
                8
            }
        }
    }

    // Block transfer operations
    fn ldi(&mut self, memory: &mut Memory) -> u8 {
        let byte = memory.read(self.hl());
        memory.write(self.de(), byte);

        self.set_hl(self.hl().wrapping_add(1));
        self.set_de(self.de().wrapping_add(1));
        self.set_bc(self.bc().wrapping_sub(1));

        let n = byte.wrapping_add(self.a);
        self.set_flag_h(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.bc() != 0);
        self.set_flag_y((n & 0x02) != 0);
        self.set_flag_x((n & 0x08) != 0);

        16
    }

    fn ldir(&mut self, memory: &mut Memory) -> u8 {
        let byte = memory.read(self.hl());
        memory.write(self.de(), byte);

        self.set_hl(self.hl().wrapping_add(1));
        self.set_de(self.de().wrapping_add(1));
        self.set_bc(self.bc().wrapping_sub(1));

        let n = byte.wrapping_add(self.a);
        self.set_flag_h(false);
        self.set_flag_n(false);
        self.set_flag_y((n & 0x02) != 0);
        self.set_flag_x((n & 0x08) != 0);

        if self.bc() != 0 {
            self.pc = self.pc.wrapping_sub(2);
            self.set_flag_pv(true);
            return 21;
        }

        self.set_flag_pv(false);
        16
    }

    fn ldd(&mut self, memory: &mut Memory) -> u8 {
        let byte = memory.read(self.hl());
        memory.write(self.de(), byte);

        self.set_hl(self.hl().wrapping_sub(1));
        self.set_de(self.de().wrapping_sub(1));
        self.set_bc(self.bc().wrapping_sub(1));

        let n = byte.wrapping_add(self.a);
        self.set_flag_h(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.bc() != 0);
        self.set_flag_y((n & 0x02) != 0);
        self.set_flag_x((n & 0x08) != 0);

        16
    }

    fn lddr(&mut self, memory: &mut Memory) -> u8 {
        let byte = memory.read(self.hl());
        memory.write(self.de(), byte);

        self.set_hl(self.hl().wrapping_sub(1));
        self.set_de(self.de().wrapping_sub(1));
        self.set_bc(self.bc().wrapping_sub(1));

        let n = byte.wrapping_add(self.a);
        self.set_flag_h(false);
        self.set_flag_n(false);
        self.set_flag_y((n & 0x02) != 0);
        self.set_flag_x((n & 0x08) != 0);

        if self.bc() != 0 {
            self.pc = self.pc.wrapping_sub(2);
            self.set_flag_pv(true);
            return 21;
        }

        self.set_flag_pv(false);
        16
    }

    // Block search operations
    fn cpi(&mut self, memory: &Memory) -> u8 {
        let val = memory.read(self.hl());
        let result = self.a.wrapping_sub(val);

        self.set_hl(self.hl().wrapping_add(1));
        self.set_bc(self.bc().wrapping_sub(1));

        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_h((self.a & 0x0F) < (val & 0x0F));
        self.set_flag_n(true);
        self.set_flag_pv(self.bc() != 0);

        let n = result.wrapping_sub(if self.get_flag_h() { 1 } else { 0 });
        self.set_flag_y((n & 0x02) != 0);
        self.set_flag_x((n & 0x08) != 0);

        16
    }

    fn cpir(&mut self, memory: &Memory) -> u8 {
        let val = memory.read(self.hl());
        let result = self.a.wrapping_sub(val);

        self.set_hl(self.hl().wrapping_add(1));
        self.set_bc(self.bc().wrapping_sub(1));

        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_h((self.a & 0x0F) < (val & 0x0F));
        self.set_flag_n(true);

        let n = result.wrapping_sub(if self.get_flag_h() { 1 } else { 0 });
        self.set_flag_y((n & 0x02) != 0);
        self.set_flag_x((n & 0x08) != 0);

        if self.bc() != 0 && result != 0 {
            self.pc = self.pc.wrapping_sub(2);
            self.set_flag_pv(true);
            return 21;
        }

        self.set_flag_pv(self.bc() != 0);
        16
    }

    fn cpd(&mut self, memory: &Memory) -> u8 {
        let val = memory.read(self.hl());
        let result = self.a.wrapping_sub(val);

        self.set_hl(self.hl().wrapping_sub(1));
        self.set_bc(self.bc().wrapping_sub(1));

        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_h((self.a & 0x0F) < (val & 0x0F));
        self.set_flag_n(true);
        self.set_flag_pv(self.bc() != 0);

        let n = result.wrapping_sub(if self.get_flag_h() { 1 } else { 0 });
        self.set_flag_y((n & 0x02) != 0);
        self.set_flag_x((n & 0x08) != 0);

        16
    }

    fn cpdr(&mut self, memory: &Memory) -> u8 {
        let val = memory.read(self.hl());
        let result = self.a.wrapping_sub(val);

        self.set_hl(self.hl().wrapping_sub(1));
        self.set_bc(self.bc().wrapping_sub(1));

        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_h((self.a & 0x0F) < (val & 0x0F));
        self.set_flag_n(true);

        let n = result.wrapping_sub(if self.get_flag_h() { 1 } else { 0 });
        self.set_flag_y((n & 0x02) != 0);
        self.set_flag_x((n & 0x08) != 0);

        if self.bc() != 0 && result != 0 {
            self.pc = self.pc.wrapping_sub(2);
            self.set_flag_pv(true);
            return 21;
        }

        self.set_flag_pv(self.bc() != 0);
        16
    }

    // Block I/O operations (stubs for now - will implement with I/O controller)
    fn ini(&mut self, _memory: &mut Memory) -> u8 {
        // IN (HL), (C); INC HL; DEC B
        self.set_hl(self.hl().wrapping_add(1));
        self.b = self.b.wrapping_sub(1);
        self.set_flag_z(self.b == 0);
        self.set_flag_n(true);
        16
    }

    fn inir(&mut self, _memory: &mut Memory) -> u8 {
        self.set_hl(self.hl().wrapping_add(1));
        self.b = self.b.wrapping_sub(1);
        self.set_flag_n(true);

        if self.b != 0 {
            self.pc = self.pc.wrapping_sub(2);
            self.set_flag_z(false);
            return 21;
        }

        self.set_flag_z(true);
        16
    }

    fn ind(&mut self, _memory: &mut Memory) -> u8 {
        self.set_hl(self.hl().wrapping_sub(1));
        self.b = self.b.wrapping_sub(1);
        self.set_flag_z(self.b == 0);
        self.set_flag_n(true);
        16
    }

    fn indr(&mut self, _memory: &mut Memory) -> u8 {
        self.set_hl(self.hl().wrapping_sub(1));
        self.b = self.b.wrapping_sub(1);
        self.set_flag_n(true);

        if self.b != 0 {
            self.pc = self.pc.wrapping_sub(2);
            self.set_flag_z(false);
            return 21;
        }

        self.set_flag_z(true);
        16
    }

    fn outi(&mut self, _memory: &Memory) -> u8 {
        self.set_hl(self.hl().wrapping_add(1));
        self.b = self.b.wrapping_sub(1);
        self.set_flag_z(self.b == 0);
        self.set_flag_n(true);
        16
    }

    fn otir(&mut self, _memory: &Memory) -> u8 {
        self.set_hl(self.hl().wrapping_add(1));
        self.b = self.b.wrapping_sub(1);
        self.set_flag_n(true);

        if self.b != 0 {
            self.pc = self.pc.wrapping_sub(2);
            self.set_flag_z(false);
            return 21;
        }

        self.set_flag_z(true);
        16
    }

    fn outd(&mut self, _memory: &Memory) -> u8 {
        self.set_hl(self.hl().wrapping_sub(1));
        self.b = self.b.wrapping_sub(1);
        self.set_flag_z(self.b == 0);
        self.set_flag_n(true);
        16
    }

    fn otdr(&mut self, _memory: &Memory) -> u8 {
        self.set_hl(self.hl().wrapping_sub(1));
        self.b = self.b.wrapping_sub(1);
        self.set_flag_n(true);

        if self.b != 0 {
            self.pc = self.pc.wrapping_sub(2);
            self.set_flag_z(false);
            return 21;
        }

        self.set_flag_z(true);
        16
    }

    // 16-bit load/store operations
    fn ld_rr_nn_indirect(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let addr = self.fetch_word(memory);
        let val = memory.read_word(addr);

        match (opcode >> 4) & 0x03 {
            0 => self.set_bc(val),
            1 => self.set_de(val),
            2 => self.set_hl(val),
            3 => self.sp = val,
            _ => unreachable!(),
        }

        20
    }

    fn ld_nn_indirect_rr(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let addr = self.fetch_word(memory);

        let val = match (opcode >> 4) & 0x03 {
            0 => self.bc(),
            1 => self.de(),
            2 => self.hl(),
            3 => self.sp,
            _ => unreachable!(),
        };

        memory.write_word(addr, val);
        20
    }

    fn sbc_hl_rr(&mut self, opcode: u8) -> u8 {
        let hl = self.hl();

        let operand = match (opcode >> 4) & 0x03 {
            0 => self.bc(),
            1 => self.de(),
            2 => self.hl(),
            3 => self.sp,
            _ => unreachable!(),
        };

        let carry = if self.get_flag_c() { 1u16 } else { 0u16 };
        let result = hl.wrapping_sub(operand).wrapping_sub(carry);

        let full_sub = (hl as u32)
            .wrapping_sub(operand as u32)
            .wrapping_sub(carry as u32);

        self.set_hl(result);
        self.set_flag_c(full_sub > 0xFFFF);
        self.set_flag_n(true);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x8000) != 0);
        self.set_flag_h(((hl & 0x0FFF) as i32 - (operand & 0x0FFF) as i32 - carry as i32) < 0);
        self.set_flag_pv(((hl ^ operand) & (hl ^ result) & 0x8000) != 0);
        self.set_flag_x(((result >> 8) & 0x08) != 0);
        self.set_flag_y(((result >> 8) & 0x20) != 0);

        15
    }

    fn adc_hl_rr(&mut self, opcode: u8) -> u8 {
        let rr = match opcode {
            0x4A => self.bc(),
            0x5A => self.de(),
            0x6A => self.hl(),
            0x7A => self.sp,
            _ => unreachable!(),
        };

        let hl = self.hl();
        let carry = if self.get_flag_c() { 1 } else { 0 };
        let result = hl.wrapping_add(rr).wrapping_add(carry);
        self.set_hl(result);

        self.set_flag_s((result & 0x8000) != 0);
        self.set_flag_z(result == 0);
        self.set_flag_h(((hl & 0x0FFF) + (rr & 0x0FFF) + carry) > 0x0FFF);
        self.set_flag_pv(((hl ^ rr) & 0x8000) == 0 && ((hl ^ result) & 0x8000) != 0);
        self.set_flag_n(false);
        self.set_flag_c((hl as u32 + rr as u32 + carry as u32) > 0xFFFF);
        self.set_flag_x(((result >> 8) & 0x08) != 0);
        self.set_flag_y(((result >> 8) & 0x20) != 0);

        15
    }

    // Register operations
    fn ld_r_a(&mut self) -> u8 {
        self.r = self.a;
        9
    }

    fn ld_i_a(&mut self) -> u8 {
        self.i = self.a;
        9
    }

    fn ld_a_r(&mut self) -> u8 {
        self.a = self.r;
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_z(self.a == 0);
        self.set_flag_h(false);
        self.set_flag_pv(self.iff2);
        self.set_flag_n(false);
        self.set_flag_x((self.a & 0x08) != 0);
        self.set_flag_y((self.a & 0x20) != 0);
        9
    }

    fn ld_a_i(&mut self) -> u8 {
        self.a = self.i;
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_z(self.a == 0);
        self.set_flag_h(false);
        self.set_flag_pv(self.iff2);
        self.set_flag_n(false);
        self.set_flag_x((self.a & 0x08) != 0);
        self.set_flag_y((self.a & 0x20) != 0);
        9
    }

    // I/O port operations (stubs - will implement with I/O controller)
    fn in_r_c(&mut self, opcode: u8) -> u8 {
        // For now, just read 0xFF
        let val = 0xFF;

        let reg = (opcode >> 3) & 0x07;
        match reg {
            0 => self.b = val,
            1 => self.c = val,
            2 => self.d = val,
            3 => self.e = val,
            4 => self.h = val,
            5 => self.l = val,
            6 => {} // IN (C) - read but don't store
            7 => self.a = val,
            _ => unreachable!(),
        }

        self.set_flag_s((val & 0x80) != 0);
        self.set_flag_z(val == 0);
        self.set_flag_h(false);
        self.set_flag_pv(val.count_ones() % 2 == 0);
        self.set_flag_n(false);
        self.set_flag_x((val & 0x08) != 0);
        self.set_flag_y((val & 0x20) != 0);

        12
    }

    fn out_c_r(&mut self, opcode: u8) -> u8 {
        // For now, just a stub
        let _reg = (opcode >> 3) & 0x07;
        12
    }

    // Interrupt mode operations
    fn im_0(&mut self) -> u8 {
        self.interrupt_mode = 0;
        8
    }

    fn im_1(&mut self) -> u8 {
        self.interrupt_mode = 1;
        8
    }

    fn im_2(&mut self) -> u8 {
        self.interrupt_mode = 2;
        8
    }

    // Negate
    fn neg(&mut self) -> u8 {
        let a = self.a;
        let result = 0u8.wrapping_sub(a);

        self.a = result;

        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_z(result == 0);
        self.set_flag_h((0 & 0x0F) < (a & 0x0F));
        self.set_flag_pv(a == 0x80);
        self.set_flag_n(true);
        self.set_flag_c(a != 0);
        self.set_flag_x((result & 0x08) != 0);
        self.set_flag_y((result & 0x20) != 0);

        8
    }

    // Rotate operations
    fn rrd(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.hl();
        let val = memory.read(addr);

        let low_a = self.a & 0x0F;
        let low_val = val & 0x0F;
        let high_val = val >> 4;

        self.a = (self.a & 0xF0) | high_val;
        memory.write(addr, (low_a << 4) | low_val);

        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_z(self.a == 0);
        self.set_flag_h(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_n(false);
        self.set_flag_x((self.a & 0x08) != 0);
        self.set_flag_y((self.a & 0x20) != 0);

        18
    }

    fn rld(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.hl();
        let val = memory.read(addr);

        let low_a = self.a & 0x0F;
        let low_val = val & 0x0F;
        let high_val = val >> 4;

        self.a = (self.a & 0xF0) | low_val;
        memory.write(addr, (high_val << 4) | low_a);

        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_z(self.a == 0);
        self.set_flag_h(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_n(false);
        self.set_flag_x((self.a & 0x08) != 0);
        self.set_flag_y((self.a & 0x20) != 0);

        18
    }

    // Return from interrupt
    fn retn(&mut self, memory: &Memory) -> u8 {
        self.iff1 = self.iff2;
        self.pc = self.pop(memory);
        14
    }

    fn reti(&mut self, memory: &Memory) -> u8 {
        self.iff1 = self.iff2;
        self.pc = self.pop(memory);
        14
    }
}
