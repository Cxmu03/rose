#![no_std]
#![no_main]

use core::panic::PanicInfo;

use vga::*;

mod vga;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut vga_buffer = VGA_BUFFER_LOCK.write();
    vga_buffer.clear();

    let message: &[u8] = b"Hello from Rose";

    let row = (VGA_HEIGHT / 2) - 1;
    let col = ((VGA_WIDTH - message.len()) / 2) - 1;

    for (index, character) in message.iter().enumerate() {
        let vga_char = VgaChar::new(*character, VgaColor::Blue, VgaColor::White);
        *vga_buffer.get_mut(col + index, row) = vga_char;
    }

    loop {}
}
