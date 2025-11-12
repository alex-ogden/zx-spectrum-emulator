#include "memory.h"

Memory::Memory() { reset(); }
Memory::~Memory() {}

void Memory::reset() {
  std::print("Starting memory reset routine");
  ram.fill(0);
  std::println("Ending memory reset routine");
}

uint8_t Memory::fetch_byte(uint16_t address) const { return ram[address]; }
uint16_t Memory::fetch_word(uint16_t address) const {
  return fetch_byte(address) | (fetch_byte(address + 1) << 8);
}
void Memory::write_byte(uint16_t address, uint8_t value) {
  if (address < 0x4000)
    return;
  ram[address] = value;
}
void Memory::write_word(uint16_t address, uint16_t value) {
  if (address < 0x4000)
    return;
  write_byte(address, (uint8_t)value);
  write_byte(address + 1, (uint8_t)(value >> 8));
}

bool Memory::load_rom(const std::string &filename, uint16_t start_address) {
  std::ifstream file(filename, std::ios::binary);

  if (!file) {
    std::println(stderr, "Failed to open ROM: {}", filename);
    return false;
  }

  // Read ROM into vector
  std::vector<uint8_t> rom_data((std::istreambuf_iterator<char>(file)),
                                std::istreambuf_iterator<char>());

  // Ensure ROM fits in memory
  if (start_address + rom_data.size() > 0x4000) {
    std::println(stderr, "ROM too large for memory. Expected {}, got: {}",
                 0x4000, (start_address + rom_data.size()));
    return false;
  }

  // Copy ROM into memory
  std::copy(rom_data.begin(), rom_data.end(), ram.begin() + start_address);

  std::println("Loaded {} bytes from {} at address 0x{:04X}", rom_data.size(),
               filename, start_address);
  return true;
}
