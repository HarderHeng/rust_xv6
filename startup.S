.section .text._start
.global _start

_start:
    # 设置栈指针：QEMU virt 的 DRAM 从 0x80000000 开始，分配 16KB 栈
    li sp, 0x80000000 + 0x4000

    # 直接跳转到 Rust 主函数
    call rust_main

    # 如果返回（不应该），进入死循环
hang:
    wfi
    j hang