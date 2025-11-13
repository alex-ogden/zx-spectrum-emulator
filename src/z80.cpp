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

uint8_t Z80::fetch_byte() { return memory.fetch_byte(PC++); }
uint16_t Z80::fetch_word() {
  uint8_t lo = fetch_byte();
  uint8_t hi = fetch_byte();
  return (hi << 8) | lo;
}

uint8_t Z80::read_byte(uint16_t address) { return memory.fetch_byte(address); }
uint16_t Z80::read_word(uint16_t address) { return memory.fetch_word(address); }
void Z80::write_byte(uint16_t address, uint8_t value) {
  memory.write_byte(address, value);
}
void Z80::write_word(uint16_t address, uint16_t value) {
  memory.write_word(address, value);
}

bool Z80::get_flag(uint8_t flag) const { return (F & flag) != 0; }
void Z80::set_flag(uint8_t flag, bool state) {
  F = (F & ~flag) | (flag * state);
}

void Z80::set_flag_c(bool state) { set_flag(FLAG_C, state); }
void Z80::set_flag_n(bool state) { set_flag(FLAG_N, state); }
void Z80::set_flag_pv(bool state) { set_flag(FLAG_PV, state); }
void Z80::set_flag_h(bool state) { set_flag(FLAG_H, state); }
void Z80::set_flag_z(bool state) { set_flag(FLAG_Z, state); }
void Z80::set_flag_s(bool state) { set_flag(FLAG_S, state); }

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
  switch (opcode) {
  // 8-bit loads (mem -> reg)
  case 0x3E:
    A = fetch_byte();
    return 7; // LD A, n
  case 0x06:
    B = fetch_byte();
    return 7; // LD B, n
  case 0x0E:
    C = fetch_byte();
    return 7; // LD C, n
  case 0x16:
    D = fetch_byte();
    return 7; // LD D, n
  case 0x1E:
    E = fetch_byte();
    return 7; // LD E, n
  case 0x26:
    H = fetch_byte();
    return 7; // LD H, n
  case 0x2E:
    L = fetch_byte();
    return 7; // LD L, n

  // 8-bit loads (reg -> reg)
  case 0x7F:
    A = A;
    return 4; // LD A, A
  case 0x78:
    A = B;
    return 4; // LD A, B
  case 0x79:
    A = C;
    return 4; // LD A, C
  case 0x7A:
    A = D;
    return 4; // LD A, D
  case 0x7B:
    A = E;
    return 4; // LD A, E
  case 0x7C:
    A = H;
    return 4; // LD A, H
  case 0x7D:
    A = L;
    return 4; // LD A, L

  // 8-bit arithmetic
  case 0x80:
    add_a(B);
    return 4; // ADD A, B
  case 0x81:
    add_a(C);
    return 4; // ADD A, C
  case 0x82:
    add_a(D);
    return 4; // ADD A, D
  case 0x83:
    add_a(E);
    return 4; // ADD A, E
  case 0x84:
    add_a(F);
    return 4; // ADD A, F
  case 0x85:
    add_a(H);
    return 4; // ADD A, H
  case 0xC6:
    add_a(L);
    return 4; // ADD A, L

  // 16-bit loads
  case 0x01:
    set_bc(fetch_word());
    return 10; // LD BC, nn
  case 0x11:
    set_de(fetch_word());
    return 10; // LD DE, nn
  case 0x21:
    set_hl(fetch_word());
    return 10; // LD HL, nn
  case 0x31:
    SP = fetch_word();
    return 10; // LD SP, nn

  // Control flow
  case 0x00:
    return 4; // NOP
  case 0x76:  /* !TODO: HALT */
    return 4; // HALT
  case 0xC3:
    PC = fetch_word();
    return 10; // JP nn

  // Extended instruction sets
  case 0xCB:
    return execute_cb_instruction(fetch_byte());
  case 0xED:
    return execute_ed_instruction(fetch_byte());
  case 0xDD:
    return execute_dd_instruction(fetch_byte());
  case 0xFD:
    return execute_fd_instruction(fetch_byte());

  default:
    std::println(stderr, "Unknown opcode: 0x{:02X} at PC: 0x{:04X}", opcode,
                 PC - 1);
    return 4;
  }
}

uint8_t Z80::execute_cb_instruction(uint8_t subopcode) {}
uint8_t Z80::execute_ed_instruction(uint8_t subopcode) {}
uint8_t Z80::execute_dd_instruction(uint8_t subopcode) {}
uint8_t Z80::execute_fd_instruction(uint8_t subopcode) {}
