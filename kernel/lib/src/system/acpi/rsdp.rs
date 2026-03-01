use core::slice;

const RSDP_SIGNATURE: [char; 8] = ['R', 'S', 'D', ' ', 'P', 'T', 'R', ' '];
const RSDP_V1_LEN: usize = 20;

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct RsdpTable {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_address: u32,

    // V2
    length: u32,
    xsdt_address: u64,
    ext_checksum: u8,
    _reserved: [u8; 3],
}

pub enum RsdpError {
    InvalidSignature([char; 8]),
    InvalidChecksum(u8),
}

impl RsdpTable {
    pub fn new(rsdp_addr: u64) -> RsdpTable {
        let ptr = rsdp_addr as *const RsdpTable;
        unsafe { ptr.read_unaligned() }
    }
    pub fn validate(&self) -> Result<(), RsdpError> {
        let signature = self.build_signature();
        if signature != RSDP_SIGNATURE {
            return Err(RsdpError::InvalidSignature(signature));
        }

        let sum = self.checksum();
        if sum != 0 {
            return Err(RsdpError::InvalidChecksum(sum));
        }
        Ok(())
    }
    pub fn revision(&self) -> u8 {
        self.revision
    }
    pub fn rsdt_address(&self) -> u32 {
        self.rsdt_address
    }
    pub fn xsdt_address(&self) -> u64 {
        self.xsdt_address
    }
    fn build_signature(&self) -> [char; 8] {
        let mut signature: [char; 8] = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
        self.signature
            .iter()
            .enumerate()
            .for_each(|(i, b)| signature[i] = char::from(*b));
        signature
    }
    fn checksum(&self) -> u8 {
        // Sets length based on rsdp version
        let length = if self.revision > 0 {
            self.length as usize
        } else {
            RSDP_V1_LEN
        };
        let bytes = unsafe { slice::from_raw_parts(self as *const RsdpTable as *const u8, length) };
        bytes.iter().fold(0u8, |sum, &byte| sum.wrapping_add(byte))
    }
}
