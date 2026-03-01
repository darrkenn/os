#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct SDTHeader {
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

impl SDTHeader {
    pub fn signature(&self) -> [u8; 4] {
        self.signature
    }
    pub fn length(&self) -> u32 {
        self.length
    }
}
