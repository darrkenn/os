use core::{ops::Add, ptr};

use crate::system::acpi::sdt::SdtHeader;

#[repr(C, packed)]
pub struct XSDT {
    h: SdtHeader,
}

pub struct XSDTRegion {
    pub table: XSDT,
    pub xsdt_address: u64,
}

impl XSDTRegion {
    pub fn new(xsdt_addr: u64) -> Self {
        let ptr = xsdt_addr as *const XSDT;
        Self {
            table: unsafe { ptr.read_unaligned() },
            xsdt_address: xsdt_addr,
        }
    }

    // Calculates the entries count by the table length field - the size of the SdtHeader struct
    // and divides that by 8.
    pub fn entries_count(&self) -> usize {
        (self.table.h.length() as usize - size_of::<SdtHeader>()) / 8
    }

    // Selects an xsdt entry by an index.
    pub fn get_entry_by_index(&self, idx: usize) -> u64 {
        let entry = unsafe { ptr::read_unaligned((self.data_address() as *const u64).add(idx)) };
        entry
    }
    // Selects an xsdt entry by matching the signature, if no matching entry is found it will
    // return None.
    pub fn get_entry_by_signature(&self, signature: [u8; 4]) -> Option<u64> {
        for i in 0..self.entries_count() {
            let addr = self.get_entry_by_index(i);
            let header = SdtHeader::from_addr(addr);
            if header.validate_signature(signature) {
                return Some(addr);
            }
        }
        None
    }
    fn data_address(&self) -> usize {
        self.xsdt_address as usize + size_of::<SdtHeader>()
    }
}

impl XSDT {
    pub fn header(&self) -> SdtHeader {
        self.h
    }
}
