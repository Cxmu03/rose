pub(crate) mod idt;
pub(crate) mod segment;

pub(crate) enum PrivilegeLevel {
    Ring0 = 0b00,
    Ring1 = 0b01,
    Ring2 = 0b10,
    Ring3 = 0b11
}