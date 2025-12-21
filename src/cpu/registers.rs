use super::Cpu;
use crate::memory::Memory;

// Flag bit positions
const FLAG_C: u8 = 0x01; // Carry
const FLAG_N: u8 = 0x02; // Add/Subtract
const FLAG_PV: u8 = 0x04; // Parity/Overflow
const FLAG_Y: u8 = 0x08; // Undocumented (bit 3 of result)
const FLAG_H: u8 = 0x10; // Half Carry
const FLAG_X: u8 = 0x20; // Undocumented (bit 5 of result)
const FLAG_Z: u8 = 0x40; // Zero
const FLAG_S: u8 = 0x80; // Sign

// Register and flag helper methods
impl Cpu {
    // == Flag-setting helper functions == //
    #[inline]
    pub fn set_flag_c(&mut self, val: bool) {
        if val {
            self.f |= FLAG_C;
        } else {
            self.f &= !FLAG_C;
        }
    }

    #[inline]
    pub fn set_flag_n(&mut self, val: bool) {
        if val {
            self.f |= FLAG_N;
        } else {
            self.f &= !FLAG_N;
        }
    }

    #[inline]
    pub fn set_flag_pv(&mut self, val: bool) {
        if val {
            self.f |= FLAG_PV;
        } else {
            self.f &= !FLAG_PV;
        }
    }

    #[inline]
    pub fn set_flag_y(&mut self, val: bool) {
        if val {
            self.f |= FLAG_Y;
        } else {
            self.f &= !FLAG_Y;
        }
    }

    #[inline]
    pub fn set_flag_h(&mut self, val: bool) {
        if val {
            self.f |= FLAG_H;
        } else {
            self.f &= !FLAG_H;
        }
    }

    #[inline]
    pub fn set_flag_x(&mut self, val: bool) {
        if val {
            self.f |= FLAG_X;
        } else {
            self.f &= !FLAG_X;
        }
    }

    #[inline]
    pub fn set_flag_z(&mut self, val: bool) {
        if val {
            self.f |= FLAG_Z;
        } else {
            self.f &= !FLAG_Z;
        }
    }

    #[inline]
    pub fn set_flag_s(&mut self, val: bool) {
        if val {
            self.f |= FLAG_S;
        } else {
            self.f &= !FLAG_S;
        }
    }

    // == Flag-getting helper functions == //
    #[inline]
    pub fn get_flag_c(&self) -> bool {
        (self.f & FLAG_C) != 0
    }

    #[inline]
    pub fn get_flag_n(&self) -> bool {
        (self.f & FLAG_N) != 0
    }

    #[inline]
    pub fn get_flag_pv(&self) -> bool {
        (self.f & FLAG_PV) != 0
    }

    #[inline]
    pub fn get_flag_y(&self) -> bool {
        (self.f & FLAG_Y) != 0
    }

    #[inline]
    pub fn get_flag_h(&self) -> bool {
        (self.f & FLAG_H) != 0
    }

    #[inline]
    pub fn get_flag_x(&self) -> bool {
        (self.f & FLAG_X) != 0
    }

    #[inline]
    pub fn get_flag_z(&self) -> bool {
        (self.f & FLAG_Z) != 0
    }

    #[inline]
    pub fn get_flag_s(&self) -> bool {
        (self.f & FLAG_S) != 0
    }

    // == Register pair helper functions == //
    #[inline]
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    #[inline]
    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = val as u8;
    }

    #[inline]
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    #[inline]
    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    #[inline]
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    #[inline]
    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }

    #[inline]
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    #[inline]
    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    // == Read & Write to registers == //
    #[inline]
    pub fn read_reg(&self, reg_code: u8, memory: &Memory) -> u8 {
        match reg_code {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            6 => memory.read(self.hl()), // (HL) - special case
            7 => self.a,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn write_reg(&mut self, reg_code: u8, val: u8, memory: &mut Memory) {
        match reg_code {
            0 => self.b = val,
            1 => self.c = val,
            2 => self.d = val,
            3 => self.e = val,
            4 => self.h = val,
            5 => self.l = val,
            6 => memory.write(self.hl(), val), // (HL) - special case
            7 => self.a = val,
            _ => unreachable!(),
        }
    }

    // == Stack operations == //
    #[inline]
    pub fn push(&mut self, val: u16, memory: &mut Memory) {
        self.sp = self.sp.wrapping_sub(2);
        memory.write_word(self.sp, val);
    }

    #[inline]
    pub fn pop(&mut self, memory: &Memory) -> u16 {
        let val = memory.read_word(self.sp);
        self.sp = self.sp.wrapping_add(2);
        val
    }
}
