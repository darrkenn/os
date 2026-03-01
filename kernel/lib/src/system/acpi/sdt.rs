pub mod sdt_header_signatures {
    // X S D T
    pub const XSDT: [u8; 4] = [88, 83, 68, 84];
}

pub enum SdtHeaderError {
    InvalidSignature([u8; 4]),
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct SdtHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

impl SdtHeader {
    pub fn validate_signature(&self, signature: [u8; 4]) -> bool {
        if self.signature != signature {
            false
        } else {
            true
        }
    }
    pub fn signature(&self) -> [u8; 4] {
        self.signature
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn revision(self) -> u8 {
        self.revision
    }
    pub fn from_addr(addr: u64) -> Self {
        let ptr = addr as *const SdtHeader;
        unsafe { ptr.read_unaligned() }
    }
}
