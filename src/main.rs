#![no_std]
#![no_main]

use core::panic::PanicInfo;

const UART0: usize = 0x10000000;

// UART 寄存器偏移
const RHR: usize = 0; // 接收 (read)
const THR: usize = 0; // 发送 (write)
const IER: usize = 1; // 中断使能
const FCR: usize = 2; // FIFO 控制
const LCR: usize = 3; // 线路控制
const MCR: usize = 4; // 调制解调器控制
const LSR: usize = 5; // 线路状态

fn uart_init() {
    let base = UART0 as *mut u8;
    // 1. 设置 LCR[7] = 1 (进入 DLAB 模式)
    unsafe { base.add(LCR).write_volatile(0x80); }
    // 2. 设置波特率 = 0 (任意值，QEMU 不关心)
    unsafe { base.add(0).write_volatile(0); } // DLL
    unsafe { base.add(1).write_volatile(0); } // DLM
    // 3. 退出 DLAB，设置 8N1: LCR = 0x03
    unsafe { base.add(LCR).write_volatile(0x03); }
    // 4. 启用 FIFO
    unsafe { base.add(FCR).write_volatile(0x07); }
    // 5. 禁用中断
    unsafe { base.add(IER).write_volatile(0); }
}

fn uart_putc(c: u8) {
    let lsr = UART0 as *const u8;
    let thr = UART0 as *mut u8;
    // 等待发送缓冲区空 (LSR bit 5 = 1)
    while (unsafe { lsr.add(LSR).read_volatile() } & 0x20) == 0 {}
    unsafe { thr.add(THR).write_volatile(c); }
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    uart_init();
    let msg = b"Hello from Rust!\n";
    for &b in msg {
        uart_putc(b);
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}