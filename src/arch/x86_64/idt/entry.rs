use crate::arch::x86_64::segment::SegmentSelector;
use super::InterruptHandler;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub(crate) struct IdtEntry {
    handler_pointer_low: u16,
    gdt_selector: SegmentSelector,
    options: IdtEntryOptions,
    handler_pointer_middle: u16,
    handler_pointer_high: u32,
    reserved: u32,
}

impl IdtEntry {
    pub(crate) fn new(segment_selector: SegmentSelector, handler_func: InterruptHandler) -> Self {
        let handler_func_pointer = handler_func as u64;

        IdtEntry {
            handler_pointer_low: handler_func_pointer as u16,
            handler_pointer_middle: (handler_func_pointer >> 16) as u16,
            handler_pointer_high: (handler_func_pointer >> 32) as u32,
            gdt_selector: segment_selector,
            options: IdtEntryOptions::new(),
            reserved: 0
        }
    }

    pub(crate) fn missing() -> Self {
        IdtEntry {
            handler_pointer_low: 0,
            handler_pointer_middle: 0,
            handler_pointer_high: 0,
            gdt_selector: SegmentSelector(0),
            options: IdtEntryOptions::minimal(),
            reserved: 0
        }
    }

    pub(crate) fn set_options(&mut self, options: IdtEntryOptions) {
        self.options = options;
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct IdtEntryOptions(u16);

impl IdtEntryOptions {
    pub(crate) fn minimal() -> Self {
        IdtEntryOptions(0x0E00)
    }

    pub(crate) fn new() -> Self {
        let mut options = IdtEntryOptions::minimal();

        options.set_interrupts_enabled(false);
        options.set_present(true);

        options
    }

    pub(crate) fn set_stack_table_index(&mut self, value: u8) {
        assert!(value < 8);

        self.0 = (self.0 & 0xFFF8) | (value as u16);
    }

    pub(crate) fn set_present(&mut self, present: bool) {
        self.0 = (self.0 & !0x8000) | ((present as u16) << 15);
    }

    pub(crate) fn set_interrupts_enabled(&mut self, enabled: bool) {
        self.0 = (self.0 & !(1 << 8)) | ((!enabled as u16) << 8);
    }

    pub(crate) fn set_privilege_level(&mut self, privilege_level: u8) {
        assert!(privilege_level < 4);

        self.0 = (self.0 & !(0b11 << 13)) | ((privilege_level as u16) << 13);
    }
}