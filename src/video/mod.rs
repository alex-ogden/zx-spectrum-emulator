use crate::cpu::Cpu;
use crate::memory::Memory;
use minifb::{Window, WindowOptions};

const SPECTRUM_SCREEN_WIDTH: usize = 256;
const SPECTRUM_SCREEN_HEIGHT: usize = 192;
const SPECTRUM_SCREEN_SF: usize = 2; // Scale factor
const SPECTRUM_DEBUG_PANEL_WIDTH: usize = 320;

// Font data for debug text
const FONT_WIDTH: usize = 5;
const FONT_HEIGHT: usize = 7;
const FONT_SCALE: usize = 2;

// ZX Spectrum colour palette (BRIGHT=0)
const SPECTRUM_COLOURS: [u32; 8] = [
    0xFF000000, // Black
    0xFF0000CD, // Blue
    0xFFCD0000, // Red
    0xFFCD00CD, // Magenta
    0xFF00CD00, // Green
    0xFF00CDCD, // Cyan
    0xFFCDCD00, // Yellow
    0xFFCDCDCD, // White
];

// ZX Spectrum colour palette (BRIGHT=1)
const SPECTRUM_COLOURS_BRIGHT: [u32; 8] = [
    0xFF000000, // Black (same)
    0xFF0000FF, // Blue (bright)
    0xFFFF0000, // Red (bright)
    0xFFFF00FF, // Magenta (bright)
    0xFF00FF00, // Green (bright)
    0xFF00FFFF, // Cyan (bright)
    0xFFFFFF00, // Yellow (bright)
    0xFFFFFFFF, // White (bright)
];

pub struct Video {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    debug_enabled: bool,
    border_colour: u8,
}

impl Video {
    pub fn new(debug_enabled: bool) -> Result<Self, minifb::Error> {
        let screen_width = SPECTRUM_SCREEN_WIDTH * SPECTRUM_SCREEN_SF;
        let screen_height = SPECTRUM_SCREEN_HEIGHT * SPECTRUM_SCREEN_SF;

        let total_width = if debug_enabled {
            screen_width + SPECTRUM_DEBUG_PANEL_WIDTH
        } else {
            screen_width
        };

        let total_height = screen_height;

        let window = Window::new(
            "ZX Spectrum 48K Emulator",
            total_width,
            total_height,
            WindowOptions::default(),
        )?;
        let buffer = vec![0; total_width * total_height];

        println!("Window size: {} x {}", total_width, total_height);
        println!("Buffer size: {} pixels", buffer.len());
        if debug_enabled {
            println!(
                "Debug panel enabled: {} x {} pixels",
                SPECTRUM_DEBUG_PANEL_WIDTH, total_height
            );
        }

        Ok(Self {
            window,
            buffer,
            width: total_width,
            height: total_height,
            debug_enabled,
            border_colour: 7, // White border by default
        })
    }

    pub fn border_colour(&self) -> u8 {
        self.border_colour
    }

    pub fn set_border_colour(&mut self, colour: u8) {
        self.border_colour = colour & 0x07;
    }

    pub fn render(&mut self, memory: &Memory, cpu: &Cpu) {
        // Clear buffer with border colour
        let border_rgb = SPECTRUM_COLOURS[self.border_colour as usize];
        self.buffer.fill(border_rgb);

        // Get screen bitmap and attributes from memory
        let bitmap = memory.screen_bitmap();
        let attributes = memory.screen_attributes();

        let scale = SPECTRUM_SCREEN_SF;

        // Render the screen using the Spectrum's quirky memory layout
        for y in 0..192 {
            for x in 0..32 {
                // Calculate bitmap address using Spectrum's interleaved layout
                // Address = 010T TSSS LLLC CCCC
                // T = third of screen (0-2)
                // S = scan line within third (0-7)
                // L = line within character (0-7)
                // C = column (0-31)

                let third = y / 64; // Which third (0-2)
                let line_in_third = y % 64; // Line within third (0-63)
                let scan = line_in_third / 8; // Which character row (0-7)
                let pixel_line = line_in_third % 8; // Pixel row within character (0-7)

                let bitmap_addr = (third << 11) | (pixel_line << 8) | (scan << 5) | x;
                let bitmap_byte = bitmap[bitmap_addr];

                // Attribute address is simpler: just character row and column
                let attr_row = y / 8;
                let attr_addr = (attr_row * 32) + x;
                let attr_byte = attributes[attr_addr];

                // Decode attribute byte
                // Bit 7: FLASH
                // Bit 6: BRIGHT
                // Bits 5-3: PAPER (background)
                // Bits 2-0: INK (foreground)
                let flash = (attr_byte & 0x80) != 0;
                let bright = (attr_byte & 0x40) != 0;
                let paper = (attr_byte >> 3) & 0x07;
                let ink = attr_byte & 0x07;

                // Get colours from palette
                let palette = if bright {
                    &SPECTRUM_COLOURS_BRIGHT
                } else {
                    &SPECTRUM_COLOURS
                };

                let ink_colour = palette[ink as usize];
                let paper_colour = palette[paper as usize];

                // TODO: Implement FLASH (toggle ink/paper every 16 frames)
                // For now, ignore flash
                let _flash = flash;

                // Render 8 pixels
                for bit in 0..8 {
                    let pixel = (bitmap_byte >> (7 - bit)) & 1;
                    let colour = if pixel == 1 { ink_colour } else { paper_colour };

                    // Draw scaled pixel
                    for sy in 0..scale {
                        for sx in 0..scale {
                            let screen_x = (x * 8 + bit) * scale + sx;
                            let screen_y = y * scale + sy;
                            let index = screen_y * self.width + screen_x;
                            if index < self.buffer.len() {
                                self.buffer[index] = colour;
                            }
                        }
                    }
                }
            }
        }

        // Render debug panel if enabled
        if self.debug_enabled {
            self.render_debug_panel(cpu, memory);
        }
    }

    fn render_debug_panel(&mut self, cpu: &Cpu, memory: &Memory) {
        let panel_x = SPECTRUM_SCREEN_WIDTH * SPECTRUM_SCREEN_SF;
        let colour = 0xFFFFFFFF; // White debug text
        let bg_colour = 0xFF1A1A1A; // Dark grey background

        // Fill debug panel background
        for y in 0..self.height {
            for x in panel_x..self.width {
                let index = y * self.width + x;
                if index < self.buffer.len() {
                    self.buffer[index] = bg_colour;
                }
            }
        }

        let mut y_pos = 10;
        let x_offset = panel_x + 10;

        // Title header
        self.draw_text("=== ZX SPECTRUM ===", x_offset, y_pos, colour);
        y_pos += 20 * FONT_SCALE;

        // CPU registers
        self.draw_text("REGISTERS:", x_offset, y_pos, colour);
        y_pos += 12 * FONT_SCALE;

        self.draw_text(&format!("PC: 0x{:04X}", cpu.pc), x_offset, y_pos, colour);
        y_pos += 10 * FONT_SCALE;
        self.draw_text(&format!("SP: 0x{:04X}", cpu.sp), x_offset, y_pos, colour);
        y_pos += 10 * FONT_SCALE;
        self.draw_text(
            &format!("A: 0x{:02X}    F: {:02X}", cpu.a, cpu.f),
            x_offset,
            y_pos,
            colour,
        );
        y_pos += 10 * FONT_SCALE;
        self.draw_text(
            &format!("B: 0x{:02X}    C: {:02X}", cpu.b, cpu.c),
            x_offset,
            y_pos,
            colour,
        );
        y_pos += 10 * FONT_SCALE;
        self.draw_text(
            &format!("D: 0x{:02X}    E: {:02X}", cpu.d, cpu.e),
            x_offset,
            y_pos,
            colour,
        );
        y_pos += 10 * FONT_SCALE;
        self.draw_text(
            &format!("H: 0x{:02X}    L: {:02X}", cpu.h, cpu.l),
            x_offset,
            y_pos,
            colour,
        );
        y_pos += 15 * FONT_SCALE;

        // Flags
        self.draw_text("FLAGS:", x_offset, y_pos, colour);
        y_pos += 12 * FONT_SCALE;

        let flags = format!(
            "S:{} Z:{} H:{} P:{} N:{} C:{}",
            if cpu.get_flag_s() { "1" } else { "0" },
            if cpu.get_flag_z() { "1" } else { "0" },
            if cpu.get_flag_h() { "1" } else { "0" },
            if cpu.get_flag_pv() { "1" } else { "0" },
            if cpu.get_flag_n() { "1" } else { "0" },
            if cpu.get_flag_c() { "1" } else { "0" },
        );
        self.draw_text(&flags, x_offset, y_pos, colour);
        y_pos += 15 * FONT_SCALE;

        // Index Registers
        self.draw_text("INDEX REGS:", x_offset, y_pos, colour);
        y_pos += 12 * FONT_SCALE;
        self.draw_text(&format!("IX: {:04X}", cpu.ix), x_offset, y_pos, colour);
        y_pos += 10 * FONT_SCALE;
        self.draw_text(&format!("IY: {:04X}", cpu.iy), x_offset, y_pos, colour);
        y_pos += 15 * FONT_SCALE;

        // Current Instruction
        self.draw_text("CURRENT OPCODE:", x_offset, y_pos, colour);
        y_pos += 12 * FONT_SCALE;
        let opcode = memory.read(cpu.pc);
        self.draw_text(
            &format!("[{:04X}]: {:02X}", cpu.pc, opcode),
            x_offset,
            y_pos,
            colour,
        );
        y_pos += 15 * FONT_SCALE;

        // Stack preview
        self.draw_text("STACK (top 4):", x_offset, y_pos, colour);
        y_pos += 12 * FONT_SCALE;
        for i in 0..4 {
            let addr = cpu.sp.wrapping_add(i * 2);
            let val = memory.read_word(addr);
            self.draw_text(
                &format!("[{:04X}]: {:04X}", addr, val),
                x_offset,
                y_pos,
                colour,
            );
            y_pos += 10 * FONT_SCALE;
        }
        y_pos += 5 * FONT_SCALE;

        // Spectrum system info
        self.draw_text("SYSTEM:", x_offset, y_pos, colour);
        y_pos += 12 * FONT_SCALE;
        self.draw_text(
            &format!("Border: {}", self.border_colour),
            x_offset,
            y_pos,
            colour,
        );
        y_pos += 15 * FONT_SCALE;

        // Interrupt state
        self.draw_text("INTERRUPTS:", x_offset, y_pos, colour);
        y_pos += 12 * FONT_SCALE;
        self.draw_text(
            &format!(
                "IFF1:{} IFF2:{} IM:{}",
                if cpu.iff1 { "1" } else { "0" },
                if cpu.iff2 { "1" } else { "0" },
                cpu.interrupt_mode
            ),
            x_offset,
            y_pos,
            colour,
        );
        y_pos += 10 * FONT_SCALE;
        self.draw_text(
            &format!("I: {:02X}  R: {:02X}", cpu.i, cpu.r),
            x_offset,
            y_pos,
            colour,
        );
    }

    fn draw_text(&mut self, text: &str, x: usize, y: usize, colour: u32) {
        for (i, ch) in text.chars().enumerate() {
            self.draw_char(ch, x + i * ((FONT_WIDTH * FONT_SCALE) + 1), y, colour);
        }
    }

    fn draw_char(&mut self, ch: char, x: usize, y: usize, colour: u32) {
        let glyph = get_font_glyph(ch);

        for row in 0..FONT_HEIGHT {
            for col in 0..FONT_WIDTH {
                if (glyph[row] >> (4 - col)) & 1 != 0 {
                    for sy in 0..FONT_SCALE {
                        for sx in 0..FONT_SCALE {
                            let px = x + (col * FONT_SCALE) + sx;
                            let py = y + (row * FONT_SCALE) + sy;
                            let index = py * self.width + px;
                            if index < self.buffer.len() {
                                self.buffer[index] = colour;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn update(&mut self) -> Result<(), minifb::Error> {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)?;
        Ok(())
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn get_keys(&self) -> Vec<minifb::Key> {
        self.window.get_keys()
    }
}

// Font set for ASCII chars (keeping your existing font data)
fn get_font_glyph(ch: char) -> [u8; 7] {
    match ch {
        ' ' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '=' => [0x00, 0x00, 0x1F, 0x00, 0x1F, 0x00, 0x00],
        '-' => [0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00],
        ':' => [0x00, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x00],
        '(' => [0x00, 0x04, 0x08, 0x08, 0x08, 0x04, 0x00],
        ')' => [0x00, 0x08, 0x04, 0x04, 0x04, 0x08, 0x00],
        '[' => [0x00, 0x0E, 0x08, 0x08, 0x08, 0x0E, 0x00],
        ']' => [0x00, 0x0E, 0x02, 0x02, 0x02, 0x0E, 0x00],
        '0' => [0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E],
        '1' => [0x04, 0x0C, 0x04, 0x04, 0x04, 0x04, 0x0E],
        '2' => [0x0E, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1F],
        '3' => [0x1F, 0x02, 0x04, 0x02, 0x01, 0x11, 0x0E],
        '4' => [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02],
        '5' => [0x1F, 0x10, 0x1E, 0x01, 0x01, 0x11, 0x0E],
        '6' => [0x06, 0x08, 0x10, 0x1E, 0x11, 0x11, 0x0E],
        '7' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
        '8' => [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E],
        '9' => [0x0E, 0x11, 0x11, 0x0F, 0x01, 0x02, 0x0C],
        'A' => [0x0E, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'B' => [0x1E, 0x11, 0x11, 0x1E, 0x11, 0x11, 0x1E],
        'C' => [0x0E, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0E],
        'D' => [0x1E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1E],
        'E' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x1F],
        'F' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x10],
        'G' => [0x0E, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0F],
        'H' => [0x11, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'I' => [0x0E, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0E],
        'J' => [0x07, 0x02, 0x02, 0x02, 0x02, 0x12, 0x0C],
        'K' => [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11],
        'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1F],
        'M' => [0x11, 0x1B, 0x15, 0x15, 0x11, 0x11, 0x11],
        'N' => [0x11, 0x11, 0x19, 0x15, 0x13, 0x11, 0x11],
        'O' => [0x0E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'P' => [0x1E, 0x11, 0x11, 0x1E, 0x10, 0x10, 0x10],
        'Q' => [0x0E, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0D],
        'R' => [0x1E, 0x11, 0x11, 0x1E, 0x14, 0x12, 0x11],
        'S' => [0x0F, 0x10, 0x10, 0x0E, 0x01, 0x01, 0x1E],
        'T' => [0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
        'U' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'V' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x0A, 0x04],
        'W' => [0x11, 0x11, 0x11, 0x15, 0x15, 0x1B, 0x11],
        'X' => [0x11, 0x11, 0x0A, 0x04, 0x0A, 0x11, 0x11],
        'Y' => [0x11, 0x11, 0x0A, 0x04, 0x04, 0x04, 0x04],
        'Z' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1F],
        _ => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    }
}
