.section .text._start
.global _start

.equ UART_BASE, 0x10000000
.equ LSR,       5
.equ THR,       0

_start:
    # 初始化栈指针（指向栈顶，假设 16KB 栈）
    li sp, 0x80000000 + 0x4000  # 16KB 栈空间

    # 初始化 UART（简化版，适用于 QEMU）
    li t0, UART_BASE
    li t1, 0x03        # 8N1
    sb t1, 3(t0)       # LCR
    li t1, 0x07        # enable FIFO
    sb t1, 2(t0)       # FCR

    # 跳转到 Rust 的 main 函数
    call rust_main

    # 如果返回，进入死循环
hang:
    wfi
    j hang