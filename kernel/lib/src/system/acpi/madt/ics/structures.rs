#[repr(C, packed)]
pub struct ProcessorLocalAPIC {
    _type: u8,
    length: u8,
    acpi_processor_uid: u8,
    apic_id: u8,
    flags: u32,
}

#[repr(C, packed)]
pub struct IOAPIC {
    _type: u8,
    length: u8,
    io_apic_id: u8,
    _reserved: u8,
    io_apic_address: u32,
    gsib: u32,
}

#[repr(C, packed)]
pub struct InterruptSourceOverride {
    _type: u8,
    length: u8,
    bus: u8,
    source: u8,
    gsi: u8,
    flags: u16,
}

#[repr(C, packed)]
pub struct NMISource {
    _type: u8,
    length: u8,
    flags: u16,
    gsi: u32,
}

#[repr(C, packed)]
pub struct LocalAPICNMI {
    _type: u8,
    length: u8,
    flags: u16,
    gsi: u32,
}

#[repr(C, packed)]
pub struct LocalAPICAddressOverride {
    _type: u8,
    length: u8,
    _reserved: u16,
    local_apic_address: u64,
}

#[repr(C, packed)]
pub struct IOSAPIC {
    _type: u8,
    length: u8,
    io_apic_id: u8,
    _reserved: u8,
    gsib: u32,
    io_sapic_address: u64,
}

#[repr(C, packed)]
pub struct LocalSAPIC {
    _type: u8,
    length: u8,
    acpi_processer_id: u8,
    local_sapic_id: u8,
    local_sapic_eid: u8,
    _reserved: [u8; 3],
    flags: u32,
    acpi_proc_uid_string: u8,
}
