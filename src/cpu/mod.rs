use crate::memory::Memory;

mod cb_instructions;
mod dd_instructions;
mod ed_instructions;
mod fd_instructions;
mod instructions;
mod registers;

pub struct Cpu {
    // Z80 CPU @ 3.5MHz (Spectrum vs 3.25MHz ZX81)
    // Registers
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    // Shadow registers
    pub af_shadow: u16,
    pub bc_shadow: u16,
    pub de_shadow: u16,
    pub hl_shadow: u16,

    // Index registers
    pub ix: u16,
    pub iy: u16,

    // Interrupt and memory registers
    pub i: u8,
    pub r: u8,

    // Interrupt flip-flops
    pub iff1: bool,
    pub iff2: bool,

    // Interrupt mode (0, 1, or 2)
    pub interrupt_mode: u8,

    // Stack pointer and program counter
    pub sp: u16,
    pub pc: u16,

    // Holds whether the CPU is halted
    pub is_halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            af_shadow: 0,
            bc_shadow: 0,
            de_shadow: 0,
            hl_shadow: 0,
            ix: 0,
            iy: 0,
            i: 0,
            r: 0,
            iff1: false,
            iff2: false,
            interrupt_mode: 0,
            sp: 0xFFFF,
            pc: 0x0000,
            is_halted: false,
        }
    }

    fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let addr = self.pc;
        let byte = memory.read(addr);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    fn fetch_word(&mut self, memory: &Memory) -> u16 {
        let lo = self.fetch_byte(memory) as u16;
        let hi = self.fetch_byte(memory) as u16;
        (hi << 8) | lo
    }

    pub fn step(&mut self, memory: &mut Memory) -> u8 {
        // Increment refresh register (R) on each M1 cycle
        self.r = (self.r & 0x80) | ((self.r + 1) & 0x7F);

        // Is the system halted?
        if self.is_halted {
            // Return 4 T-states for a NOP-equivalent halt cycle
            return 4;
        }

        // Retrieve the opcode at the current program counter
        // PC is incremented in fetch_byte automatically
        let opcode = self.fetch_byte(memory);
        let cycles = self.execute(opcode, memory);
        cycles
    }

    fn execute(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        self.execute_instruction(opcode, memory)
    }
}
