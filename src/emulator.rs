use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::video::Video;

pub struct Emulator {
    cpu: Cpu,
    memory: Memory,
    video: Video,
    cycles: u64,
    frame_cycles: u64,
}

// Spectrum timing constants
const CYCLES_PER_FRAME: u64 = 69888; // 3.5MHz / 50H

// This var will be required later
#[allow(dead_code)]
const CYCLES_PER_SCANLINE: u64 = 224; // 69888 / 312 lines

impl Emulator {
    pub fn new(rom: Vec<u8>, debug_enabled: bool) -> Result<Self, minifb::Error> {
        Ok(Self {
            cpu: Cpu::new(),
            memory: Memory::new(rom),
            video: Video::new(debug_enabled)?,
            cycles: 0,
            frame_cycles: 0,
        })
    }

    pub fn step(&mut self) -> u8 {
        let cycles = self.cpu.step(&mut self.memory);
        self.cycles += cycles as u64;
        self.frame_cycles += cycles as u64;

        // Check if we've completed a frame
        if self.frame_cycles >= CYCLES_PER_FRAME {
            self.frame_cycles -= CYCLES_PER_FRAME;
            // TODO: Generate interrupt here when interrupts are implemented
        }

        cycles
    }

    pub fn run_frame(&mut self) {
        let target_cycles = self.cycles + CYCLES_PER_FRAME;

        while self.cycles < target_cycles && !self.cpu.is_halted {
            self.step();
        }
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn cpu_mut(&mut self) -> &mut Cpu {
        &mut self.cpu
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    pub fn video(&self) -> &Video {
        &self.video
    }

    pub fn video_mut(&mut self) -> &mut Video {
        &mut self.video
    }

    pub fn is_halted(&self) -> bool {
        self.cpu.is_halted
    }

    pub fn total_cycles(&self) -> u64 {
        self.cycles
    }

    pub fn update_display(&mut self) -> Result<(), minifb::Error> {
        self.video.update()
    }

    pub fn is_window_open(&self) -> bool {
        self.video.is_open()
    }

    pub fn render_display(&mut self) -> Result<(), minifb::Error> {
        self.video.render(&self.memory, &self.cpu);
        self.video.update()
    }

    pub fn update_keyboard(&mut self) {
        let keys = self.video.get_keys();
        // TODO: Update keyboard state when I/O controller is implemented
        let _ = keys;
    }

    pub fn set_border_colour(&mut self, colour: u8) {
        self.video.set_border_colour(colour);
    }

    // Helper to write directly to screen memory for testing
    pub fn write_to_screen(&mut self, x: usize, y: usize, pattern: u8) {
        if x < 32 && y < 192 {
            let third = y / 64;
            let line_in_third = y % 64;
            let scan = line_in_third / 8;
            let pixel_line = line_in_third % 8;

            let bitmap_addr = 0x4000 + (third << 11) | (pixel_line << 8) | (scan << 5) | x;
            self.memory.write(bitmap_addr as u16, pattern);
        }
    }

    // Helper to set screen attributes
    pub fn set_attribute(&mut self, x: usize, y: usize, attr: u8) {
        if x < 32 && y < 24 {
            let attr_addr = 0x5800 + (y * 32) + x;
            self.memory.write(attr_addr as u16, attr);
        }
    }

    // Clear screen to a specific colour
    pub fn clear_screen(&mut self, ink: u8, paper: u8, bright: bool) {
        self.memory.clear_screen(ink, paper, bright, false);
    }

    pub fn dump_system_info(&mut self) {
        println!("\n=== ZX Spectrum 48K System Info ===");
        println!("Total cycles: {}", self.cycles);
        println!("Frame cycles: {}/{}", self.frame_cycles, CYCLES_PER_FRAME);
        println!("CPU PC: 0x{:04X}", self.cpu.pc);
        println!("CPU SP: 0x{:04X}", self.cpu.sp);
        println!("Border colour: {}", self.video_mut().border_colour());

        // Check screen memory
        let bitmap_start = 0x4000;
        let attr_start = 0x5800;
        println!("Screen bitmap at: 0x{:04X}", bitmap_start);
        println!("Screen attributes at: 0x{:04X}", attr_start);

        // Sample first few bytes of screen
        print!("First 16 bitmap bytes: ");
        for i in 0..16 {
            print!("{:02X} ", self.memory.read(bitmap_start + i));
        }
        println!();

        print!("First 16 attribute bytes: ");
        for i in 0..16 {
            print!("{:02X} ", self.memory.read(attr_start + i));
        }
        println!();

        println!("===================================\n");
    }
}
