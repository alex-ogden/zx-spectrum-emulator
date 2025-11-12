#include "z80.h"
#include "memory.h"

Z80::Z80(Memory &mem) : memory(mem) { reset(); }
Z80::~Z80() {}

void Z80::reset() {
  A = F = B = C = D = E = H = L = 0;
  A_ = F_ = B_ = C_ = D_ = E_ = H_ = L_ = 0;
  I = R = 0;
  IX = IY = 0;
  PC = 0x0000;
  SP = 0xFFFF;
  IFF1 = IFF2 = false;
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

void Z80::emulate_cycle() {
  uint16_t opcode = fetch_byte();
  execute_instruction(opcode);
}

void Z80::execute_instruction(uint8_t opcode) {}
