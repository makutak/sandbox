#pragma once

#include <cstdint>
#include <array>

#include "error.hpp"

namespace pci {

/** CONFIG_ADDRESS レジスタの IO ポートアドレス*/
const uint16_t kConfigAddress = 0x0cf8;
/** CONFIG_DATA レジスタの IO ポートアドレス*/
const uint16_t kConfigData = 0x0cfc;

struct Device {
  uint8_t bus, device, function, header_type;
};

inline std::array<Device, 32> devices;
inline int num_device;

Error ScanAllBus();
}
