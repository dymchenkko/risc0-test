/*
  Copyright 2022 Risc0, Inc.

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/

OUTPUT_FORMAT("elf32-littleriscv", "elf32-littleriscv", "elf32-littleriscv")
OUTPUT_ARCH(riscv)
ENTRY(_start)
EXTERN(__start)

/* Must match risc0/zkvm/platform/memory.h and zkvm/sdk/rust/platform/src/memory.rs */
/* Write-only section must match the range hardcoded in the circuit. */
MEMORY {
  stack        : ORIGIN = 0x00000000, LENGTH =   9M
  data    (RW) : ORIGIN = 0x00900000, LENGTH =   1M
  heap         : ORIGIN = 0x00A00000, LENGTH =  20M
  input        : ORIGIN = 0x01E00000, LENGTH =   1M
  gpio         : ORIGIN = 0x01F00000, LENGTH =   1M
  prog    (X)  : ORIGIN = 0x02000000, LENGTH =  10M
  sha          : ORIGIN = 0x02A00000, LENGTH =   1M
  wom          : ORIGIN = 0x02B00000, LENGTH =  21M
}

SECTIONS {
  .text : {
    *(.text._start)
    *(.text.__start)
    *(.text*)
    *(.rodata*)
    *(.srodata*)
  } >prog

  .data : {
    *(.data .data.*)
    *(.gnu.linkonce.d.*)
    __global_pointer$ = . + 0x800;
    *(.sdata .sdata.* .sdata2.*)
    *(.gnu.linkonce.s.*)
  } >data

  . = ALIGN(4);

  .bss (NOLOAD) :  {
    __bss_begin = .;
    *(.sbss*)
    *(.gnu.linkonce.sb.*)
    *(.bss .bss.*)
    *(.gnu.linkonce.b.*)
    *(COMMON)
    . = ALIGN(4);
    __result = .;
    /* Result is 9 words * 4 = 36 bytes, 8 words for output, and 1 word for output size*/
    __bss_end = . + 36;
  } >data

  __bss_size = __bss_end - __bss_begin;

  __heap_start = ORIGIN(heap);
  __heap_end = __heap_start + LENGTH(heap);
  __heap_size = LENGTH(heap);

  __stack_init$ = ORIGIN(stack) + LENGTH(stack) - 4;

  /DISCARD/ : {
    *(.rel*)
    *(.comment)
    *(.eh_frame)
    *(.riscv.attributes)
  }
}
