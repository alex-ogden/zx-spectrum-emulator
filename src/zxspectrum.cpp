#include "zxspectrum.h"
#include "memory.h"
#include "ula.h"
#include "z80.h"

#include <cstdint>

ZXSpectrum::ZXSpectrum(Memory &mem, Z80 &cpu) : memory(mem), cpu(cpu) {}
ZXSpectrum::~ZXSpectrum() {}

void ZXSpectrum::run() {
  const uint32_t CYCLES_PER_FRAME = 69888;
  uint32_t cycles_this_frame = 0;

  running = true;
  cpu.set_halted(false);

  while (running) {
    uint8_t cycles = cpu.emulate_cycle();
    cycles_this_frame += cycles;

    if (cycles_this_frame >= CYCLES_PER_FRAME) {
      cycles_this_frame -= CYCLES_PER_FRAME;

      // !TODO: Implement cpu interrupt
      // cpu.interrupt();

      // !TODO: Implement ULA
      // ula.render_display();
    }
  }
}

void ZXSpectrum::stop() { running = false; }
