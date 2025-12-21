mod ram;
mod rom;
pub use rom::load_rom;

pub struct Memory {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Memory {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
            ram: vec![0; 0xC000], // 48KB RAM (0x4000 -> 0xFFFF)
        }
    }

    pub fn rom(&self) -> &[u8] {
        &self.rom
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[addr as usize],
            0x4000..=0xFFFF => {
                let offset = (addr - 0x4000) as usize;
                self.ram[offset]
            }
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let lo = self.read(addr) as u16;
        let hi = self.read(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x3FFF => {} // ROM is not writable
            0x4000..=0xFFFF => {
                let offset = (addr - 0x4000) as usize;
                self.ram[offset] = val;
            }
        }
    }

    pub fn write_word(&mut self, addr: u16, val: u16) {
        let hi = (val >> 8) as u8;
        let lo = val as u8;

        self.write(addr, lo);
        self.write(addr.wrapping_add(1), hi);
    }

    pub fn screen_bitmap(&self) -> &[u8] {
        &self.ram[0..0x1800] // 0x4000 -> 0x57FF
    }

    pub fn screen_attributes(&self) -> &[u8] {
        &self.ram[0x1800..0x1B00] // 0x5800 -> 0x5AFF
    }

    pub fn clear_screen(&mut self, ink: u8, paper: u8, bright: bool, flash: bool) {
        // Clear bitmap
        for i in 0..0x1800 {
            self.ram[i] = 0;
        }

        // Set attributes
        let attr = (if flash { 0x80 } else { 0 })
            | (if bright { 0x40 } else { 0 })
            | ((paper & 0x07) << 3)
            | (ink & 0x07);

        for i in 0x1800..0x1B00 {
            self.ram[i] = attr;
        }
    }
}
