/** @file pci.cpp
 *
 * PCI バス制御のプログラムを集めたファイル。
 */

/** @brief CONFIG_ADDRESS 用の 32 ビット整数を生成する */
#include "pci.hpp"

#include "asmfunc.h"


uint32_t MakeAddress(uint8_t bus, uint8_t device, uint8_t function,
                     uint8_t reg_addr) {
  auto shl = [](uint32_t x, unsigned int bits) {
               return  x << bits;
             };
  return shl(1, 31) // enable bit
      | shl(bus, 16)
      | shl(device, 11)
      | shl(function, 8)
      | (reg_addr & 0xfcu);
}

void WriteAddress(uint32_t address) {
  IoOut32(kConfigAddress, address);
}

void WriteData(uint32_t value) {
  IoOut32(kConfigData, value);
}

uint32_t ReadData() {
  return IoIn32(kConfigData);
}

uint16_t ReadVendorId(uint8_t bus, uint8_t device, uint8_t function) {
  WriteAddress(MakeAddress(bus, device, function, 0x00));
  return ReadData() & 0xffffu;
}
