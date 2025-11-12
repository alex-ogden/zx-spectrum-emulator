#pragma once

#include <array>
#include <print>

class Memory {
public:
  Memory();
  ~Memory();

  void reset();

private:
  std::array<uint8_t, 64 * 1024> ram;
};

void Memory::reset() {
  std::print("Starting memory reset routine");
  ram.fill(0);
  std::println("Ending memory reset routine");
}
