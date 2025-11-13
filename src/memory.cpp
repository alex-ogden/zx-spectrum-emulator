#include "memory.h"

Memory::Memory() { reset(); }
Memory::~Memory() {}

void Memory::reset() {
  std::println("Starting Memory reset routine");
  ram.fill(0);
  std::println("Ending Memory reset routine");
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

uint8_t Memory::get_contention_delay(uint16_t address, uint64_t cycles) {
  // Check if not in contended memory range
  if (address < 0x4000 || address >= 0x8000)
    return 0;

  // Calculate position within frame (assuming 69888 t-states per frame)
  uint32_t frame_cycle = cycles % 69888;

  // Check if we're in border/blanking period (no contention)
  if (frame_cycle < 14336 || frame_cycle >= 58368)
    return 0;

  // Calculate delay based on pattern
  static const std::array<uint8_t, 8> contention_pattern = {6, 5, 4, 3,
                                                            2, 1, 0, 0};
  return contention_pattern[frame_cycle % 8];
}

bool Memory::is_contended_address(uint16_t address) const {
  return (address >= 0x4000 && address < 0x8000);
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
