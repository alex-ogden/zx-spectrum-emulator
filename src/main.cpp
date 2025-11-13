#include "memory.h"
#include "z80.h"

#include <SDL2/SDL.h>
#include <chrono>
#include <filesystem>
#include <map>
#include <print>
#include <string>
#include <thread>

int main(int argc, char *argv[]) {
  Memory mem;
  Z80 cpu(mem);

  // User must provide at least a BASIC ROM
  if (argc < 2) {
    std::println(stderr, "Usage: {} <path_to_rom> [.tap file]", argv[0]);
    return 1;
  }

  std::string basic_rom = argv[1];

  if (std::filesystem::exists(basic_rom)) {
    if (!mem.load_rom(basic_rom, 0x0000)) {
      return 1;
    } // Load ROM into memory (ROM) at 0x0000
  } else {
    std::println(stderr, "Could not find file: {}", basic_rom);
    return 1;
  }

  // For now, just execute some instructions for testing
  std::println("Starting execution tests...");
  for (int i = 0; i < 10; ++i) {
    uint8_t cycles = cpu.emulate_cycle();
    std::println("PC: 0x{:04X}, Cycles: {}", cpu.get_pc(), cycles);
  }

  std::println("Tests complete");

  return 0;
}
