use crate::system::acpi::sdt::SDTHeader;

#[repr(C, packed)]
pub struct XSDT {
    h: SDTHeader,
    next_sdt: u64,
}

impl XSDT {
    pub fn new(xsdt_addr: u64) -> Self {
        let ptr = xsdt_addr as *const XSDT;
        unsafe { ptr.read_unaligned() }
    }
    pub fn header(&self) -> SDTHeader {
        self.h
    }
}
