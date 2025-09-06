use super::PrivilegeLevel;

pub(crate) enum DescriptorTable {
    Gdt = 0,
    Ldt = 1
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SegmentSelector(pub u16);

impl SegmentSelector {
    pub(crate) fn new(entry_index: u16, table: DescriptorTable, privilege_level: PrivilegeLevel) -> Self {
        assert!(entry_index < (1 << 13));

        let mut selector_value = 0;
        selector_value |= entry_index << 3;
        selector_value |= (table as u16) << 2; 
        selector_value |= privilege_level as u16;

        SegmentSelector(selector_value)
    }
}