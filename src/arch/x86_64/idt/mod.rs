use entry::{IdtEntry, IdtEntryOptions};
use lazy_static::lazy_static;
use crate::arch::x86_64::segment::SegmentSelector;
use crate::vga::println;

use core::arch::asm;
use core::mem::size_of;

pub(crate) mod entry;

lazy_static! {
    pub(crate) static ref IDT: Idt = {
        let mut idt = Idt::new();

        idt.set_handler(0, divide_by_zero_handler);

        idt
    };
}

extern "C" fn divide_by_zero_handler() -> ! {
    println!("Caught Interrupt: Divide by Zero");

    loop {}
}

type InterruptHandler = extern "C" fn () -> !;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct IdtDescriptor {
    limit: u16,
    offset: u64
}

pub(crate) struct Idt([IdtEntry; 16]);

impl Idt {
    pub(crate) fn new() -> Idt {
        Idt([IdtEntry::missing(); 16])
    }

    pub(crate) fn load(&'static self) {
        let descriptor: IdtDescriptor = IdtDescriptor {
            offset: self as *const _ as u64,
            limit: (size_of::<Self>() - 1) as u16
        };

        unsafe { 
            asm!("lidt [{}]", in(reg) &descriptor, options(nostack))
        };
    }

    pub(crate) fn set_handler_with_options(&mut self, entry_index: u8, handler: InterruptHandler, options: IdtEntryOptions) {
        let cs: u16;

        // SAFETY: Moving from cs into local variable is perfectly safe
        unsafe {
            asm!("mov {0:x}, cs", out(reg) cs, options(nomem, nostack, preserves_flags));
        }

        let mut entry = IdtEntry::new(SegmentSelector(cs), handler);
        entry.set_options(options);

        self.0[entry_index as usize] = entry;
    }

    pub(crate) fn set_handler(&mut self, entry_index: u8, handler: InterruptHandler) {
        self.set_handler_with_options(entry_index, handler, IdtEntryOptions::new());
    }
}