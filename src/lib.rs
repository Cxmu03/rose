#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn kernel_main() -> ! {
    unsafe {
        let vga_start: *mut u16 = 0xb8000 as *mut u16;

        *vga_start = 0x4F48;
    }

    loop {}
}
