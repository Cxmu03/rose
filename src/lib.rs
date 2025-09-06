#![no_std]
#![no_main]

use core::panic::PanicInfo;

use vga::*;

mod vga;
mod arch;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut vga_lock = VGA_BUFFER_LOCK.write();
    vga_lock.clear();
    vga_lock.foreground_color = VgaColor::Red;
    drop(vga_lock);

    println!("Hello from Rose");
    loop {}
}
