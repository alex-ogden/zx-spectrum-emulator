#include "z80.h"
#include "memory.h"

Z80::Z80(Memory &mem) : memory(mem) { reset(); }
Z80::~Z80() {}

void Z80::reset() {
  std::println("Starting CPU reset routine");
  A = F = B = C = D = E = H = L = 0;
  A_ = F_ = B_ = C_ = D_ = E_ = H_ = L_ = 0;
  I = R = 0;
  IX = IY = 0;
  PC = 0x0000;
  SP = 0xFFFF;
  IFF1 = IFF2 = false;
  total_cycles = 0;
  std::println("Ending CPU reset routine");
}

// Get and set halted status
bool Z80::get_halted() const { return is_halted; }
void Z80::set_halted(bool halted) { is_halted = halted; }

// Fetches byte/word and increments program counter
uint8_t Z80::fetch_byte() { return memory.fetch_byte(PC++); }
uint16_t Z80::fetch_word() {
  uint8_t lo = fetch_byte();
  uint8_t hi = fetch_byte();
  return (hi << 8) | lo;
}

// Fetches byte/word without incrementing program counter
// For instructions that retrieve a value from memory
uint8_t Z80::read_byte(uint16_t address) { return memory.fetch_byte(address); }
uint16_t Z80::read_word(uint16_t address) { return memory.fetch_word(address); }
void Z80::write_byte(uint16_t address, uint8_t value) {
  memory.write_byte(address, value);
}
void Z80::write_word(uint16_t address, uint16_t value) {
  memory.write_word(address, value);
}

// Get and set flag helper functions
bool Z80::get_flag(uint8_t flag) const { return (F & flag) != 0; }
void Z80::set_flag(uint8_t flag, bool state) {
  F = (F & ~flag) | (flag * state);
}

// Register pair helper functions
// Get:
uint16_t Z80::get_af() { return (uint16_t)A << 8 | (uint16_t)F; }
uint16_t Z80::get_bc() { return (uint16_t)B << 8 | (uint16_t)C; }
uint16_t Z80::get_de() { return (uint16_t)D << 8 | (uint16_t)E; }
uint16_t Z80::get_hl() { return (uint16_t)H << 8 | (uint16_t)L; }

// Set:
void Z80::set_af(uint16_t value) {
  A = (uint8_t)value >> 8;
  F = (uint8_t)value;
}
void Z80::set_bc(uint16_t value) {
  B = (uint8_t)value >> 8;
  C = (uint8_t)value;
}
void Z80::set_de(uint16_t value) {
  D = (uint8_t)value >> 8;
  E = (uint8_t)value;
}
void Z80::set_hl(uint16_t value) {
  H = (uint8_t)value >> 8;
  L = (uint8_t)value;
}

uint16_t Z80::get_pc() const { return PC; }
uint16_t Z80::get_sp() const { return SP; }
uint64_t Z80::get_total_cycles() const { return total_cycles; }

uint8_t Z80::emulate_cycle() {
  uint16_t opcode = fetch_byte();
  uint8_t cycles = execute_instruction(opcode);
  total_cycles += cycles;
  return cycles;
}

uint8_t Z80::read_byte_contended(uint16_t address) {
  uint8_t delay = memory.get_contention_delay(address, total_cycles);
  total_cycles += delay;
  return memory.fetch_byte(address);
}

void Z80::write_byte_contended(uint16_t address, uint8_t value) {
  uint8_t delay = memory.get_contention_delay(address, total_cycles);
  total_cycles += delay;
  memory.write_byte(address, value);
}

uint8_t Z80::execute_instruction(uint8_t opcode) {
  // clang-format off
  switch (opcode) {
  // 8-bit loads (mem -> reg)
  case 0x3E: A = fetch_byte(); return 7; // LD A, n
  case 0x06: B = fetch_byte(); return 7; // LD B, n
  case 0x0E: C = fetch_byte(); return 7; // LD C, n
  case 0x16: D = fetch_byte(); return 7; // LD D, n
  case 0x1E: E = fetch_byte(); return 7; // LD E, n
  case 0x26: H = fetch_byte(); return 7; // LD H, n
  case 0x2E: L = fetch_byte(); return 7; // LD L, n

  // 8-bit loads (reg -> reg)
  case 0x7F: A = A; return 4; // LD A, A
  case 0x78: A = B; return 4; // LD A, B
  case 0x79: A = C; return 4; // LD A, C
  case 0x7A: A = D; return 4; // LD A, D
  case 0x7B: A = E; return 4; // LD A, E
  case 0x7C: A = H; return 4; // LD A, H
  case 0x7D: A = L; return 4; // LD A, L

  // 8-bit arithmetic
  case 0x80: add_a_r(B); return 4;                    // ADD A, B
  case 0x81: add_a_r(C); return 4;                    // ADD A, C
  case 0x82: add_a_r(D); return 4;                    // ADD A, D
  case 0x83: add_a_r(E); return 4;                    // ADD A, E
  case 0x84: add_a_r(H); return 4;                    // ADD A, H
  case 0x85: add_a_r(L); return 4;                    // ADD A, L
  case 0x86: add_a_r(read_byte(get_hl())); return 7;  // ADD A, (HL)
  case 0x87: add_a_r(A); return 4;                    // ADD A, A 
  case 0xC6: add_a_r(fetch_byte()); return 7;         // ADD A, L

  // 16-bit arithmetic - opcode switched on to determine source register 
  case 0x09: add_hl_rr(opcode); return 11;
  case 0x19: add_hl_rr(opcode); return 11;
  case 0x29: add_hl_rr(opcode); return 11;
  case 0x39: add_hl_rr(opcode); return 11;

  // 16-bit loads
  case 0x01: set_bc(fetch_word()); return 10; // LD BC, nn
  case 0x11: set_de(fetch_word()); return 10; // LD DE, nn
  case 0x21: set_hl(fetch_word()); return 10; // LD HL, nn
  case 0x31: SP = fetch_word(); return 10;    // LD SP, nn

  // Enable and disable interrups
  case 0xF3: disable_interrupt(); return 4; // Disable interrupts
  case 0xFB: enable_interrupt(); return 4;  // Enable interrupts 

  // Increment/Decrement operations
    // 8-bit 
  case 0x04: inc_8bit(B); return 4; // INC B
  case 0x0C: inc_8bit(C); return 4; // INC C
  case 0x14: inc_8bit(D); return 4; // INC D
  case 0x1C: inc_8bit(E); return 4; // INC E
  case 0x24: inc_8bit(H); return 4; // INC H
  case 0x2C: inc_8bit(L); return 4; // INC L
  case 0x3C: inc_8bit(A); return 4; // INC A
  case 0x05: dec_8bit(B); return 4; // DEC B
  case 0x0D: dec_8bit(C); return 4; // DEC C
  case 0x15: dec_8bit(D); return 4; // DEC D
  case 0x1D: dec_8bit(E); return 4; // DEC E
  case 0x25: dec_8bit(H); return 4; // DEC H
  case 0x2D: dec_8bit(L); return 4; // DEC L
  case 0x3D: dec_8bit(A); return 4; // DEC A
    // 16-bit 
  case 0x03: set_bc(get_bc() + 1); return 6;  // INC BC 
  case 0x13: set_de(get_de() + 1); return 6;  // INC DE 
  case 0x23: set_hl(get_hl() + 1); return 6;  // INC HL
  case 0x33: SP++; return 6;                  // INC SP 
  case 0x0B: set_bc(get_bc() - 1); return 6;  // DEC BC 
  case 0x1B: set_de(get_de() - 1); return 6;  // DEC DE 
  case 0x2B: set_hl(get_hl() - 1); return 6;  // DEC HL
  case 0x3B: SP--; return 6;                  // DEC SP 
  case 0x34: inc_hl_indirect(); return 11;    // INC (HL)
  case 0x35: dec_hl_indirect(); return 11;    // DEC (HL)

  // Control flow
  case 0x00: return 4;                      // NOP
  case 0x76: set_halted(true); return 4;    // HALT
  case 0xC3: PC = fetch_word(); return 10;  // JP nn

  // Extended instruction sets
  case 0xCB: return execute_cb_instruction(fetch_byte());   // CB Instructions
  case 0xED: return execute_ed_instruction(fetch_byte());   // ED Instructions
  case 0xDD: return execute_dd_instruction(fetch_byte());   // DD Instructions
  case 0xFD: return execute_fd_instruction(fetch_byte());   // FD Instructions

  // Stack ops
  // PUSH
  case 0xC5: push(get_bc()); return 11;  // PUSH BC
  case 0xD5: push(get_de()); return 11;  // PUSH DE
  case 0xE5: push(get_hl()); return 11;  // PUSH HL
  case 0xF5: push(get_af()); return 11;  // PUSH AF

  // POP
  case 0xC1: set_bc(pop()); return 10;  // POP BC
  case 0xD1: set_de(pop()); return 10;  // POP DE
  case 0xE1: set_hl(pop()); return 10;  // POP HL
  case 0xF1: set_af(pop()); return 10;  // POP AF

  case 0xCD: { // CALL nn
    uint16_t address = fetch_word();
    push(PC);
    PC = address;
    return 17;
  };
  case 0xC9: PC = pop(); return 10;  // RET

  // Conditional jumps
  case 0xC2: if (!get_flag(FLAG_Z)) { PC = fetch_word(); return 10; } else { fetch_word(); return 10; };   // JP NZ, nn
  case 0xCA: if (get_flag(FLAG_Z))  { PC = fetch_word(); return 10; } else { fetch_word(); return 10; };   // JP Z, nn
  case 0xD2: if (!get_flag(FLAG_C)) { PC = fetch_word(); return 10; } else { fetch_word(); return 10; };   // JP NC, nn
  case 0xDA: if (get_flag(FLAG_C))  { PC = fetch_word(); return 10; } else { fetch_word(); return 10; };   // JP C, nn

  // Bit rotation/decimal conversion
  case 0x17: rla(); return 4;   // RLA 
  case 0x1F: rra(); return 4;   // RRA 
  case 0x07: rlca(); return 4;  // RLCA 
  case 0x0F: rrca(); return 4;  // RRCA 
  case 0x27: daa(); return 4;   // DAA 
  case 0x2F: cpl(); return 4;   // CPL 
  case 0x37: scf(); return 4;   // SCF
  case 0x3F: ccf(); return 4;   // CCF

  // Load from memory using HL
  case 0x7E: A = read_byte(get_hl()); return 7;  // LD A, (HL)
  case 0x46: B = read_byte(get_hl()); return 7;  // LD B, (HL)
  case 0x4E: C = read_byte(get_hl()); return 7;  // LD C, (HL)
  case 0x56: D = read_byte(get_hl()); return 7;  // LD D, (HL)
  case 0x5E: E = read_byte(get_hl()); return 7;  // LD E, (HL)
  case 0x66: H = read_byte(get_hl()); return 7;  // LD H, (HL)
  case 0x6E: L = read_byte(get_hl()); return 7;  // LD L, (HL)

  // Store to memory using HL
  case 0x77: write_byte(get_hl(), A); return 7;  // LD (HL), A
  case 0x70: write_byte(get_hl(), B); return 7;  // LD (HL), B
  case 0x71: write_byte(get_hl(), C); return 7;  // LD (HL), C
  case 0x72: write_byte(get_hl(), D); return 7;  // LD (HL), D
  case 0x73: write_byte(get_hl(), E); return 7;  // LD (HL), E
  case 0x74: write_byte(get_hl(), H); return 7;  // LD (HL), H
  case 0x75: write_byte(get_hl(), L); return 7;  // LD (HL), L
  case 0x36: write_byte(get_hl(), fetch_byte()); return 10;  // LD (HL), n

  // Load from/to memory using immediate addresses
  case 0x3A: A = read_byte(fetch_word()); return 13;         // LD A, (nn)
  case 0x32: write_byte(fetch_word(), A); return 13;         // LD (nn), A
  case 0x2A: set_hl(read_word(fetch_word())); return 16;     // LD HL, (nn)
  case 0x22: write_word(fetch_word(), get_hl()); return 16;  // LD (nn), HL
  
  default:
    std::println(stderr, "Unknown opcode: 0x{:02X} at PC: 0x{:04X}", opcode, PC - 1);
    return 4;
  }
  // clang-format on
}

void Z80::enable_interrupt() {
  IFF1 = true;
  IFF2 = true;
}
void Z80::disable_interrupt() {
  IFF1 = false;
  IFF2 = false;
}

void Z80::rla() {
  uint8_t bit7 = A >> 7;
  uint8_t carry_bit = (get_flag(FLAG_C)) ? 1 : 0;
  uint8_t result = (A << 1) | carry_bit;
  A = result;

  set_flag(FLAG_C, (bit7 == 1));
  set_flag(FLAG_N, false);
  set_flag(FLAG_H, false);
}
void Z80::rlca() {
  uint8_t bit7 = A >> 7;
  uint8_t result = (A << 1) | bit7;
  A = result;

  set_flag(FLAG_C, (bit7 == 1));
  set_flag(FLAG_N, false);
  set_flag(FLAG_H, false);
}
void Z80::rra() {
  uint8_t bit0 = A & 1;
  uint8_t carry_bit = (get_flag(FLAG_C)) ? 0x80 : 0;
  uint8_t result = (A >> 1) | carry_bit;
  A = result;

  set_flag(FLAG_C, (bit0 == 1));
  set_flag(FLAG_N, false);
  set_flag(FLAG_H, false);
}
void Z80::rrca() {
  uint8_t bit0 = A & 1;
  uint8_t result = (A >> 1) | (bit0 << 7);
  A = result;

  set_flag(FLAG_C, (bit0 == 1));
  set_flag(FLAG_N, false);
  set_flag(FLAG_H, false);
}
void Z80::daa() {
  uint8_t a = A;
  uint8_t correction = 0;
  bool carry_out = get_flag(FLAG_C);

  bool n = get_flag(FLAG_N);
  bool h = get_flag(FLAG_H);
  bool c = get_flag(FLAG_C);

  if (!n) {
    // After addition
    if (h || (a & 0x0F) > 9) {
      correction |= 0x06;
    }
    if (c || a > 0x99) {
      correction |= 0x60;
      carry_out = true;
    }
    A = a + correction;
  } else {
    // After subtraction
    if (h) {
      correction |= 0x06;
    }
    if (c) {
      correction |= 0x60;
    }
    A = a - correction;
  }

  // Set flags
  set_flag(FLAG_S, A & 0x80);
  set_flag(FLAG_Z, A == 0);
  set_flag(FLAG_H, false);
  set_flag(FLAG_PV, has_parity(A));
  set_flag(FLAG_C, carry_out);
  set_flag(FLAG_Y, A & 0x20);
  set_flag(FLAG_X, A & 0x08);
}

void Z80::cpl() {
  A = ~A;

  set_flag(FLAG_N, true);
  set_flag(FLAG_H, true);
  set_flag(FLAG_Y, (A & 0x20) != 0);
  set_flag(FLAG_X, (A & 0x08) != 0);
}

void Z80::scf() {
  set_flag(FLAG_C, true);
  set_flag(FLAG_H, false);
  set_flag(FLAG_N, false);
  set_flag(FLAG_Y, (A & 0x20) != 0);
  set_flag(FLAG_X, (A & 0x08) != 0);
}

void Z80::ccf() {
  // Invert the carry flag
  bool prev_carry_status = get_flag(FLAG_C);
  set_flag(FLAG_C, !get_flag(FLAG_C));

  // Other flags
  set_flag(FLAG_N, false);
  set_flag(FLAG_H, prev_carry_status);
  set_flag(FLAG_Y, (A & 0x20) != 0);
  set_flag(FLAG_X, (A & 0x08) != 0);
}

// "value" is whatever is stored in the reg passed to the function
void Z80::add_a_r(uint8_t value) {
  uint8_t old_value = A;
  uint8_t new_value = A + value;
  A = new_value;

  set_flag(FLAG_C, ((uint16_t)old_value + (uint16_t)value) > 0xFF);
  set_flag(FLAG_N, false);
  set_flag(FLAG_Z, new_value == 0);
  set_flag(FLAG_S, (new_value & 0x80) != 0);
  set_flag(FLAG_H, ((old_value & 0x0F) + (value & 0x0F)) > 0x0F);
  set_flag(FLAG_PV,
           ((old_value ^ new_value) & (value ^ new_value) & 0x80) != 0);
  set_flag(FLAG_Y, (new_value & 0x20) != 0);
  set_flag(FLAG_X, (new_value & 0x08) != 0);
}
void Z80::sub_a_r(uint8_t value) {
  uint8_t old_value = A;
  uint8_t new_value = A - value;
  A = new_value;

  set_flag(FLAG_C, value > old_value);
  set_flag(FLAG_N, true);
  set_flag(FLAG_Z, new_value == 0);
  set_flag(FLAG_S, (new_value & 0x80) != 0);
  set_flag(FLAG_H, (old_value & 0x0F) < (value & 0x0F));
  set_flag(FLAG_PV,
           ((old_value ^ value) & (old_value ^ new_value) & 0x80) != 0);
  set_flag(FLAG_Y, (new_value & 0x20) != 0);
  set_flag(FLAG_X, (new_value & 0x08) != 0);
}
void Z80::add_a_n(uint8_t value) {
  uint8_t old_value = A;
  uint8_t new_value = A + value;
  A = new_value;

  set_flag(FLAG_C, ((uint16_t)old_value + (uint16_t)value) > 0xFF);
  set_flag(FLAG_N, false);
  set_flag(FLAG_Z, new_value == 0);
  set_flag(FLAG_S, (new_value & 0x80) != 0);
  set_flag(FLAG_H, (old_value & 0x0F) + (value & 0x0F) > 0x0F);
  set_flag(FLAG_PV,
           ((old_value ^ new_value) & (value ^ new_value) & 0x80) != 0);
  set_flag(FLAG_Y, (new_value & 0x20) != 0);
  set_flag(FLAG_X, (new_value & 0x08) != 0);
}
void Z80::and_a(uint8_t value) {
  A &= value;

  set_flag(FLAG_C, false);
  set_flag(FLAG_N, false);
  set_flag(FLAG_PV, has_parity(A));
  set_flag(FLAG_H, true);
  set_flag(FLAG_Z, A == 0);
  set_flag(FLAG_S, (A & 0x80) != 0);
  set_flag(FLAG_Y, (A & 0x20) != 0);
  set_flag(FLAG_X, (A & 0x08) != 0);
}
void Z80::or_a(uint8_t value) {
  A |= value;

  set_flag(FLAG_C, false);
  set_flag(FLAG_N, false);
  set_flag(FLAG_PV, has_parity(A));
  set_flag(FLAG_H, false);
  set_flag(FLAG_Z, A == 0);
  set_flag(FLAG_S, (A & 0x80) != 0);
  set_flag(FLAG_Y, (A & 0x20) != 0);
  set_flag(FLAG_X, (A & 0x08) != 0);
}
void Z80::xor_a(uint8_t value) {
  A ^= value;

  set_flag(FLAG_C, false);
  set_flag(FLAG_N, false);
  set_flag(FLAG_PV, has_parity(A));
  set_flag(FLAG_H, false);
  set_flag(FLAG_Z, A == 0);
  set_flag(FLAG_S, (A & 0x80) != 0);
  set_flag(FLAG_Y, (A & 0x20) != 0);
  set_flag(FLAG_X, (A & 0x08) != 0);
}
void Z80::cp_a(uint8_t value) {
  // A is not modified here, just take the result
  uint8_t result = A - value;

  set_flag(FLAG_C, value > A);
  set_flag(FLAG_N, true);
  set_flag(FLAG_PV, ((A ^ value) & (A ^ result) & 0x80) != 0);
  set_flag(FLAG_H, (A & 0x0F) < (value & 0x0F));
  set_flag(FLAG_Z, result == 0); // Can also use A == value for same check
  set_flag(FLAG_S, (result & 0x80) != 0);
  set_flag(FLAG_Y, (result & 0x20) != 0);
  set_flag(FLAG_X, (result & 0x08) != 0);
}
void Z80::inc_8bit(uint8_t &reg) {
  uint8_t old_value = reg;
  reg++;

  set_flag(FLAG_N, false);
  set_flag(FLAG_Z, reg == 0);
  set_flag(FLAG_S, (reg & 0x80) != 0);
  set_flag(FLAG_H, (old_value & 0x0F) == 0x0F);
  set_flag(FLAG_PV, old_value == 0x7F);
  set_flag(FLAG_Y, (reg & 0x20) != 0);
  set_flag(FLAG_X, (reg & 0x08) != 0);
}
void Z80::dec_8bit(uint8_t &reg) {
  uint8_t old_value = reg;
  reg--;

  set_flag(FLAG_N, true);
  set_flag(FLAG_Z, reg == 0);
  set_flag(FLAG_S, (reg & 0x80) != 0);
  set_flag(FLAG_H, (old_value & 0x0F) == 0x00);
  set_flag(FLAG_PV, old_value == 0x80);
  set_flag(FLAG_Y, (reg & 0x20) != 0);
  set_flag(FLAG_X, (reg & 0x08) != 0);
}
void Z80::inc_hl_indirect() {
  uint16_t address = get_hl();
  uint8_t old_value = read_byte(address);
  uint8_t new_value = old_value + 1;
  write_byte(address, new_value);

  set_flag(FLAG_N, false);
  set_flag(FLAG_Z, new_value == 0);
  set_flag(FLAG_S, (new_value & 0x80) != 0);
  set_flag(FLAG_H, (old_value & 0x0F) == 0x0F);
  set_flag(FLAG_PV, old_value == 0x7F);
  set_flag(FLAG_Y, (new_value & 0x20) != 0);
  set_flag(FLAG_X, (new_value & 0x08) != 0);
}
void Z80::dec_hl_indirect() {
  uint16_t address = get_hl();
  uint8_t old_value = read_byte(address);
  uint8_t new_value = old_value - 1;
  write_byte(address, new_value);

  set_flag(FLAG_N, true);
  set_flag(FLAG_Z, new_value == 0);
  set_flag(FLAG_S, (new_value & 0x80) != 0);
  set_flag(FLAG_H, (old_value & 0x0F) == 0x00);
  set_flag(FLAG_PV, old_value == 0x80);
  set_flag(FLAG_Y, (new_value & 0x20) != 0);
  set_flag(FLAG_X, (new_value & 0x08) != 0);
}
void Z80::add_hl_rr(uint16_t opcode) {
  uint16_t hl = get_hl();
  uint16_t value = 0;
  // clang-format off
  switch (opcode) {
  case 0x09: value = get_bc();  break;
  case 0x19: value = get_de();  break;
  case 0x29: value = hl;        break;
  case 0x39: value = SP;        break;
  default: return;
  }
  // clang-format on

  uint32_t result = (uint32_t)hl + (uint32_t)value;

  // Flags
  set_flag(FLAG_N, false);
  set_flag(FLAG_H, ((hl & 0x0FFF) + (value & 0x0FFF)) > 0x0FFF);
  set_flag(FLAG_C, result > 0xFFFF);

  // Undocumented flags taken from high byte of result
  uint16_t new_hl = result & 0xFFFF;
  set_flag(FLAG_Y, (new_hl & 0x2000) != 0);
  set_flag(FLAG_X, (new_hl & 0x0800) != 0);

  set_hl(new_hl);
}

uint8_t Z80::execute_cb_instruction(uint8_t subopcode) { return 4; }
uint8_t Z80::execute_ed_instruction(uint8_t subopcode) { return 4; }
uint8_t Z80::execute_dd_instruction(uint8_t subopcode) { return 4; }
uint8_t Z80::execute_fd_instruction(uint8_t subopcode) { return 4; }

bool Z80::has_parity(uint8_t value) { return (std::popcount(value) & 1) == 0; }

void Z80::push(uint16_t value) {
  SP -= 2;
  write_word(SP, value);
}

uint16_t Z80::pop() {
  uint16_t value = read_word(SP);
  SP += 2;
  return value;
}
