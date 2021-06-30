/** @file pci.cpp
 *
 * PCI バス制御のプログラムを集めたファイル。
 */

/** @brief CONFIG_ADDRESS 用の 32 ビット整数を生成する */
#include "pci.hpp"

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
