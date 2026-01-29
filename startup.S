.section .text
.global _start

.equ UART_BASE, 0x10000000
.equ RHR,       0      # Receiver Holding (read)
.equ THR,       0      # Transmitter Holding (write)
.equ IER,       1      # Interrupt Enable
.equ FCR,       2      # FIFO Control
.equ LCR,       3      # Line Control
.equ MCR,       4      # Modem Control
.equ LSR,       5      # Line Status

_start:
    # 初始化 UART
    li t0, UART_BASE

    # 设置 LCR[7] = 1 (进入 DLAB 模式)
    li t1, 0x80
    sb t1, LCR(t0)

    # 设置波特率低字节 = 0
    sb zero, 0(t0)
    # 设置波特率高字节 = 0
    sb zero, 1(t0)

    # 退出 DLAB，设置 8N1: LCR = 0x03
    li t1, 0x03
    sb t1, LCR(t0)

    # 启用 FIFO
    li t1, 0x07
    sb t1, FCR(t0)

    # 禁用中断
    sb zero, IER(t0)

    # 加载字符串地址
    la t1, hello_str

print_loop:
    lbu t2, 0(t1)
    beqz t2, hang

    # 等待发送缓冲区空：检查 LSR[5] == 1
wait_tx:
    lbu t3, LSR(t0)
    andi t3, t3, 0x20
    beqz t3, wait_tx

    # 发送字符
    sb t2, THR(t0)
    addi t1, t1, 1
    j print_loop

hang:
    wfi
    j hang

hello_str:
    .asciz "Hello World!\n"