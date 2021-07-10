#pragma once

#include <cstdint>
#include <array>

#include "error.hpp"

namespace pci {


/** CONFIG_ADDRESS レジスタの IO ポートアドレス*/
const uint16_t kConfigAddress = 0x0cf8;

/** CONFIG_DATA レジスタの IO ポートアドレス*/
const uint16_t kConfigData = 0x0cfc;

struct ClassCode {
  uint16_t base, sub, interface;

  /** ベースクラスが等しい場合に真を返す */
  bool Match(uint8_t b) { return b == base; }

  /** ベースクラスとサブクラスが等しい場合に真を返す */
  bool Match(uint8_t b, uint8_t s) { return Match(b) && s == sub; }

  /** ベース、サブ、インターフェースが等しい場合に真を返す */
  bool Match(uint8_t b, uint8_t s, uint8_t i) {
    return Match(b, s) && i == interface;
  }
};

struct Device {
  uint8_t bus, device, function, header_type;
  ClassCode class_code;
};

void WriteAddress(uint32_t address);
void WriteData(uint32_t value);
uint32_t ReadData();
uint16_t ReadVendorId(uint8_t bus, uint8_t device, uint8_t function);
uint16_t ReadDeviceId(uint8_t bus, uint8_t device, uint8_t function);
uint8_t ReadHeaderType(uint8_t bus, uint8_t device, uint8_t function);
ClassCode ReadClassCode(uint8_t bus, uint8_t device, uint8_t function);
uint32_t ReadBusNumbers(uint8_t bus, uint8_t device, uint8_t function);
bool IsSingleFunctionDevice(uint8_t header_type);

inline std::array<Device, 32> devices;
inline int num_device;

Error ScanAllBus();
}
