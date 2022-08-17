// "E:\Programs\qemu\qemu-system-x86_64.exe" -drive format=raw,file="C:\Users\bjrus\Desktop\Development\Rust\rust_os\target\x86-64-rust_os\debug\bootimage-rust_os.bin"

#![no_std]
#![no_main]

use core::panic::PanicInfo;

const HELLO: &'static [u8; 12] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    loop {}
}
