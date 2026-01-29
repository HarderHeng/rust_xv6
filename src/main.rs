// main.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// 定义 UART0 基地址（QEMU virt 机器）
const UART0: usize = 0x10000000;
const UART_THR: usize = UART0 + 0x00; // Transmit Holding Register

#[no_mangle]
extern "C" fn main() -> ! {
    let hello = b"Hello, World!\r\n";
    for &byte in hello {
        uart_putc(byte);
    }

    loop {}
}

fn uart_putc(byte: u8) {
    unsafe {
        core::ptr::write_volatile(UART_THR as *mut u8, byte);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}