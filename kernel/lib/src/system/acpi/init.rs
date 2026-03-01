use core::ptr::read_unaligned;

use crate::{
    fb_println,
    system::{
        acpi::{
            fadt::{AddressType, FADT},
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

    let fadt_addr = convert_physical_to_virtual_addr(xsdt_region.get_entry_by_index(0));

    let fadt = FADT::new(fadt_addr);
    if !fadt.header().validate_signature(signatures::FADT) {
        panic!("Invalid fadt signature: {:#?}", fadt.header().signature());
    } else {
        fb_println!("Fadt validated");
    }
    match fadt.which_firmware_ctrl() {
        AddressType::Base => {
            fb_println!("FACS loaded");
            let facs_addr = fadt.firmware_ctrl_addr();
        }
        AddressType::Extended => {
            fb_println!("Extended FACS loaded");
            let facs_addr = fadt.extended_firmware_ctrl_addr();
        }
    }
}
