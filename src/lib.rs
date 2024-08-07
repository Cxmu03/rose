#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

fn clear_vga_buf() {
    for i in 0..25*80 {
        let vga_char_ptr = (0xb8000 + 2 * i) as *mut u16;

        unsafe {
            *vga_char_ptr = 0;
        }
    }
}

#[no_mangle]
pub extern fn kernel_main() -> ! {
    let hello = b"Hello from rose";
    let color_byte = 0x1f; // white foreground, blue background

    clear_vga_buf();

    let mut hello_colored = [color_byte; 30];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // write `Hello World!` to the center of the VGA text buffer
    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };

    loop {}
}
