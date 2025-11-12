#pragma once

#include <array>
#include <cstdint>
#include <fstream>
#include <print>
#include <string>
#include <vector>

class Memory {
public:
  Memory();
  ~Memory();

  void reset();

  uint8_t fetch_byte(uint16_t address) const;
  uint16_t fetch_word(uint16_t address) const;
  void write_byte(uint16_t address, uint8_t value);
  void write_word(uint16_t address, uint16_t value);

  bool load_rom(const std::string &filename, uint16_t start_address);

private:
  /* Memory layout:
        0x0000 -> 0x3FFF (16KB)     - ROM (BASIC Interpreter)
        0x4000 -> 0x57FF (6144B)    - Screen pixel data         -| ULA memory
        0x5800 -> 0x5AFF (768B)     - Screen colour attributes  -| stored here
        0x5B00 -> 0xFFFF (the rest) - General purpose RAM       */
  // 16KB ROM (0x0000 -> 0x3FFF) & 48KB RAM (0x4000 -> 0xFFFF)
  std::array<uint8_t, 64 * 1024> ram;
};
