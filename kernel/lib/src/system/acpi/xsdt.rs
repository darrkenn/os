use crate::system::acpi::sdt::SdtHeader;

#[repr(C, packed)]
pub struct XSDT {
    h: SdtHeader,
    fadt_addr: u64,
}

impl XSDT {
    pub fn new(xsdt_addr: u64) -> Self {
        let ptr = xsdt_addr as *const XSDT;
        unsafe { ptr.read_unaligned() }
    }
    pub fn header(&self) -> SdtHeader {
        self.h
    }
    pub fn fadt_addr(&self) -> u64 {
        self.fadt_addr
    }
}
