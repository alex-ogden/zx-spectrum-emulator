#include "zxspectrum.h"

#include <SDL2/SDL.h>
#include <chrono>
#include <filesystem>
#include <map>
#include <print>
#include <string>
#include <thread>

int main(int argc, char *argv[]) {
  ZXSpectrum emu;

  // User must provide at least a BASIC ROM
  if (argc < 2) {
    std::println(stderr, "Usage: {} <path_to_rom> [.tap file]", argv[0]);
    return 1;
  }

  std::string basic_rom = argv[1];

  if (std::filesystem::exists(basic_rom)) {
    return 0; // do nothing for now
  }

  return 0;
}
