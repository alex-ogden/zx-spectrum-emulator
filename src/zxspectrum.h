#pragma once

class Memory;
class Z80;

class ZXSpectrum {
public:
  ZXSpectrum(Memory &mem, Z80 &cpu);
  ~ZXSpectrum();

  void run();
  void stop();

private:
  Memory &memory;
  Z80 &cpu;

  bool running = false;
};
