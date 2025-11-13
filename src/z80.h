#pragma once

#include <cstdint>
#include <print>

class Memory;

class Z80 {
public:
  Z80(Memory &mem);
  ~Z80();

  void reset();

  // Fetch/Write to memory
  uint8_t fetch_byte();
  uint16_t fetch_word();

  uint8_t read_byte(uint16_t address);
  uint16_t read_word(uint16_t address);
  void write_byte(uint16_t address, uint8_t value);
  void write_word(uint16_t address, uint16_t value);

  // Flag bit positions on F register
  static constexpr uint8_t FLAG_C = 0x01;  // Bit 0: Carry
  static constexpr uint8_t FLAG_N = 0x02;  // Bit 1: Add/Subtract
  static constexpr uint8_t FLAG_PV = 0x04; // Bit 2: Parity/Overflow
  static constexpr uint8_t FLAG_X = 0x08;  // Bit 3: Undocumented
  static constexpr uint8_t FLAG_H = 0x10;  // Bit 4: Half-Carry
  static constexpr uint8_t FLAG_Y = 0x20;  // Bit 5: Undocumented
  static constexpr uint8_t FLAG_Z = 0x40;  // Bit 6: Zero
  static constexpr uint8_t FLAG_S = 0x80;  // Bit 7: Sign

  // Get and set flags
  bool get_flag(uint8_t flag) const;
  void set_flag(uint8_t flag, bool state);

  // Flag convenience methods
  void set_flag_c(bool state);
  void set_flag_n(bool state);
  void set_flag_pv(bool state);
  void set_flag_h(bool state);
  void set_flag_z(bool state);
  void set_flag_s(bool state);

  // Register pair methods
  // Get:
  uint16_t get_af();
  uint16_t get_bc();
  uint16_t get_de();
  uint16_t get_hl();

  // Set:
  void set_af(uint16_t value);
  void set_bc(uint16_t value);
  void set_de(uint16_t value);
  void set_hl(uint16_t value);

  // Helpers for PC/SP/Cycles etc...
  uint16_t get_pc() const;
  uint16_t get_sp() const;
  uint64_t get_total_cycles() const;

  // Cycle emulation and opcode decoding
  uint8_t emulate_cycle();

private:
  Memory &memory; // Pointer to memory

  uint8_t A, F, B, C, D, E, H, L;         // Primary registers
  uint8_t A_, F_, B_, C_, D_, E_, H_, L_; // Alternate registers
  uint8_t I;                              // Interrupt
  uint8_t R;                              // Memory refresh
  uint16_t IX, IY;                        // Index registers
  uint16_t PC;                            // Program counter
  uint16_t SP;                            // Stack pointer

  bool IFF1, IFF2; // Interrupt flip flops

  // Track the total number of cycles
  uint64_t total_cycles;

  uint8_t read_byte_contended(uint16_t address);
  void write_byte_contended(uint16_t address, uint8_t value);

  uint8_t execute_instruction(uint8_t opcode);
  uint8_t execute_cb_instruction(uint8_t subopcode);
  uint8_t execute_ed_instruction(uint8_t subopcode);
  uint8_t execute_dd_instruction(uint8_t subopcode);
  uint8_t execute_fd_instruction(uint8_t subopcode);

  // Instruction functions
  void add_a(uint8_t value);
  void sub_a(uint8_t value);
  void and_a(uint8_t value);
  void or_a(uint8_t value);
  void xor_a(uint8_t value);
  void cp_a(uint8_t value);
  void inc_8bit(uint8_t &reg);
  void dec_8bit(uint8_t &reg);
};
