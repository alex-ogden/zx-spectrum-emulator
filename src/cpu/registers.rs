pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub d: u8,
    pub c: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub a_alt: u8,
    pub f_alt: u8,
    pub b_alt: u8,
    pub d_alt: u8,
    pub c_alt: u8,
    pub e_alt: u8,
    pub h_alt: u8,
    pub l_alt: u8,

    pub ix: u16,
    pub iy: u16,
    pub sp: u16,
    pub pc: u16,

    pub i: u8,
    pub r: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            d: 0,
            c: 0,
            e: 0,
            h: 0,
            l: 0,
            a_alt: 0,
            f_alt: 0,
            b_alt: 0,
            d_alt: 0,
            c_alt: 0,
            e_alt: 0,
            h_alt: 0,
            l_alt: 0,
            ix: 0,
            iy: 0,
            sp: 0xFFFF,
            pc: 0x0000,
            i: 0,
            r: 0,
        }
    }

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val as u8) & 0xF0;
    }
    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = (val as u8);
    }
    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = (val as u8);
    }
    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = (val as u8);
    }
}
