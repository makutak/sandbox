#pragma once

#include <stdint.h>

typedef uintptr_t Elf64_Addr;
typedef uint64_t  Elf64_Off;
typedef uint16_t  Elf64_Half;
typedef uint32_t  Elf64_Word;
typedef int32_t   Elf64_Sword;
typedef uint64_t  Elf64_Xword;
typedef int64_t   Elf64_Sxword;

#define EI_NIDNET 16

typedef struct {
  unsigned char e_idnet[EI_NIDNET];
  Elf64_Half    e_type;
  Elf64_Half    e_machine;
  Elf64_Word    e_version;
  Elf64_Addr    e_entry;
  Elf64_Off     e_phoff;     // プログラムヘッダのファイルオフセット
  Elf64_Off     e_shoff;
  Elf64_Sword   e_flags;
  Elf64_Half    e_ephsize;
  Elf64_Half    e_phentsize; // 要素一つの大きさ
  Elf64_Half    e_phnum;     // 要素数
  Elf64_Half    e_shentsize;
  Elf64_Half    e_shnum;
  Elf64_Half    e_shstrndx;
} Elf64_Ehdr;
