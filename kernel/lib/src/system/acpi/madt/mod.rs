pub mod ics;
use crate::{
    fb_println,
    system::acpi::{
        madt::ics::{InterruptControllerStructureType, structures::ICSTypeLength},
        sdt::SdtHeader,
    },
};

#[repr(C, packed)]
pub struct MADT {
    h: SdtHeader,
    // Local interrupt controller address
    lic_address: u32,
    flags: u32,
    // Interrupt controller structures
}

impl MADT {
    pub fn header(&self) -> &SdtHeader {
        &self.h
    }
    pub fn lic_address(&self) -> u32 {
        self.lic_address
    }
}

pub struct MADTRegion {
    pub table: MADT,
    pub madt_address: u64,
}

impl MADTRegion {
    pub fn new(madt_addr: u64) -> Self {
        let ptr = madt_addr as *const MADT;
        Self {
            table: unsafe { ptr.read_unaligned() },
            madt_address: madt_addr,
        }
    }
    fn data_address(&self) -> u64 {
        self.madt_address + size_of::<MADT>() as u64
    }
    fn entries_length(&self) -> u32 {
        self.table.h.length() - size_of::<SdtHeader>() as u32
    }
    pub fn find_ics_of_type(&self, ics_type: InterruptControllerStructureType) -> Option<u64> {
        let entries_end_addr: u64 = self.data_address() + self.entries_length() as u64;
        let mut current_addr: u64 = self.data_address();
        loop {
            if current_addr >= entries_end_addr {
                return None;
            }

            // We cannot know the structure so we must read it as a ICSTypeLength
            let structure =
                unsafe { core::ptr::read_unaligned(current_addr as *const ICSTypeLength) };

            if structure.stype == ics_type as u8 {
                return Some(current_addr);
            }
            current_addr += structure.length as u64;
        }
    }
}
