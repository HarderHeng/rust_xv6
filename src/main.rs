#![no_std]
#![no_main]

// HTIF 的 to-host 地址（QEMU virt 特有）
const TO_HOST: *mut u64 = 0x80001000 as *mut u64;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // HTIF putchar 协议：cmd=1, data=char
    unsafe {
        for &c in b"Hello via HTIF!\n" {
            TO_HOST.write_volatile(0x01 | ((c as u64) << 8));
        }
    }
    loop {}
}