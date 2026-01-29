# startup.s
.section .text.boot
.global _start

_start:
    # 设置栈指针（SP）到 RAM 高地址
    # QEMU virt 机器的 RAM 起始于 0x80000000，大小通常为 128MB
    la sp, _stack_top

    # 跳转到 Rust main 函数
    jal ra, main

    # 如果 main 返回，则进入死循环
hang:
    j hang

.section .bss
.align 4
_stack_bottom:
    .skip 4096   # 4KB 栈空间
_stack_top: