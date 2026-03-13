use crate::{
    fb_println,
    system::{
        acpi::{
            local_apic,
            madt::{
                MADTRegion,
                ics::{self, InterruptControllerStructureType},
            },
            rsdp::{RsdpError, RsdpTable},
            signatures,
            xsdt::XSDTRegion,
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

    // Load MADT
    let madt_addr = convert_physical_to_virtual_addr(
        xsdt_region
            .get_entry_by_signature(signatures::MADT)
            .expect("Couldn't find madt"),
    );
    let madt = MADTRegion::new(madt_addr);

    // Setup CPU lcoal apic and enable timer
    let lic_addr = convert_physical_to_virtual_addr(madt.table.lic_address() as u64);
    unsafe {
        local_apic::init(lic_addr, local_apic::timer::TimerMode::Periodic);
    }
    fb_println!("Local APIC enabled and timer started");

    let io_apic_structure_ptr =
        match madt.find_ics_of_type(InterruptControllerStructureType::IOAPIC) {
            Some(addr) => addr,
            None => panic!("Cant find I/O APIC ICS"),
        } as *const ics::structures::IOAPIC;

    let io_apic_ptr = unsafe {
        convert_physical_to_virtual_addr(
            core::ptr::read_unaligned(io_apic_structure_ptr).address() as u64
        ) as *const u32
    };
    unsafe { core::ptr::write_volatile::<u32>(io_apic_ptr.cast_mut(), 0x01) };
    let id = unsafe { core::ptr::read_volatile::<u32>(io_apic_ptr.offset(4)) };

    fb_println!("{}", id);
}
