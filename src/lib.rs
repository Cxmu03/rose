#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

use vga::*;
use arch::x86_64::idt::IDT;

mod vga;
mod arch;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    IDT.load();

    let mut vga_lock = VGA_BUFFER_LOCK.write();
    vga_lock.clear();
    vga_lock.foreground_color = VgaColor::Red;
    drop(vga_lock);

    println!("Hello from Rose");

    unsafe {
        asm!("mov dx, 0");
        asm!("div dx");
    }

    loop {}
}
