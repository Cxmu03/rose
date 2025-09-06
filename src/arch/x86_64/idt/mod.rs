use entry::{IdtEntry, IdtEntryOptions};
use crate::arch::x86_64::segment::SegmentSelector;

use core::arch::asm;

mod entry;

type InterruptHandler = extern "C" fn () -> !;

struct Idt([IdtEntry; 16]);

impl Idt {
    pub(crate) fn new() -> Idt {
        Idt([IdtEntry::missing(); 16])
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