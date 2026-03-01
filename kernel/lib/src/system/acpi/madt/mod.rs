use crate::system::acpi::sdt::SdtHeader;

pub mod ics;

#[repr(C, packed)]
pub struct MADT {
    h: SdtHeader,
    // Local interrupt controller address
    lic_address: u32,
    flags: u32,
    // Interrupt controller structure
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
}
