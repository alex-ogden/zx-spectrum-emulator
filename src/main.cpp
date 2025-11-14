#include "memory.h"
#include "z80.h"
#include "zxspectrum.h"

#include <SDL2/SDL.h>
#include <filesystem>
#include <iostream>
#include <print>
#include <string>
#include <thread>

int main(int argc, char *argv[]) {
  Memory mem;
  Z80 cpu(mem);
  ZXSpectrum zxspectrum(mem, cpu);

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

  std::thread emu_thread([&]() { zxspectrum.run(); });

  std::println("Emulator running... press Enter to stop");
  std::cin.get();

  zxspectrum.stop();
  emu_thread.join();

  return 0;
}
