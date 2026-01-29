ENTRY(_start)

MEMORY
{
  RAM : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS
{
  .text : {
    *(.text.boot)
    *(.text*)
  } > RAM

  .bss : {
    *(.bss*)
    *(COMMON)
  } > RAM

  . = ALIGN(16);
  _stack_top = .;
}