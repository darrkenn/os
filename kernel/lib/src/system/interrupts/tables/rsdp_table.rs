#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct RsdpTable {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,

    // V2
    pub length: u32,
    pub xsdt_address: u64,
    pub ext_checksum: u8,
    _reserved: [u8; 3],
}

impl RsdpTable {
    pub fn new(rsdp_addr: u64) -> RsdpTable {
        let ptr = rsdp_addr as *const RsdpTable;
        unsafe { ptr.read_unaligned() }
    }
}
