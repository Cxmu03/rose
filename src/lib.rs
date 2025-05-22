#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

use vga::*;

mod vga;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    VGA_BUFFER_LOCK.write().clear();

    println!("Hello from Rose");
    loop {}
}
