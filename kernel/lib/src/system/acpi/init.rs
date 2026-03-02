use core::ptr::read_unaligned;

use crate::{
    fb_println,
    system::{
        acpi::{
            fadt::{AddressType, FADT},
            madt::{MADT, MADTRegion},
            rsdp::{RsdpError, RsdpTable},
            sdt::{SdtHeader, SdtHeaderError},
            signatures,
            xsdt::{XSDT, XSDTRegion},
        },
        physical_memory::convert_physical_to_virtual_addr,
    },
};

pub fn init(rsdp_addr: u64) {
    // Load and validate rsdp table
    let rsdp_table = RsdpTable::new(convert_physical_to_virtual_addr(rsdp_addr));
    fb_println!("Rsdp version: {}", rsdp_table.revision());
    match rsdp_table.validate() {
        Ok(_) => {
            fb_println!("Rsdp validated")
        }
        Err(e) => match e {
            RsdpError::InvalidSignature(sig) => {
                panic!("Invalid rsdp signature: {:#?}", sig);
            }
            RsdpError::InvalidChecksum(csum) => {
                panic!("Invalid v1 checksum: {:#?}", csum);
            }
        },
    }

    // Load and validate xsdt table
    let xsdt_region = XSDTRegion::new(convert_physical_to_virtual_addr(rsdp_table.xsdt_address()));
    if !xsdt_region
        .table
        .header()
        .validate_signature(signatures::XSDT)
    {
        panic!(
            "Invalid xsdt signature: {:#?}",
            xsdt_region.table.header().signature()
        )
    } else {
        fb_println!("Xsdt validated")
    };

    let madt_addr = convert_physical_to_virtual_addr(
        xsdt_region
            .get_entry_by_signature(signatures::MADT)
            .expect("Couldn't find madt"),
    );

    let madt = MADTRegion::new(madt_addr);
    let lic_addr = convert_physical_to_virtual_addr(madt.table.lic_address() as u64);
    fb_println!("{}", lic_addr);
}
