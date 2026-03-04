pub trait IcsStructure {
    fn structure_type() -> u8;
}

#[allow(dead_code)]
#[repr(u8)]
pub enum InterruptControllerStructureType {
    ProcessorLocalApic = 0,
    IOAPIC = 1,
    InterruptSourceOverride = 2,
    NMISource = 3,
    LocalAPICNMI = 4,
    // Local APIC Address Override
    LAAO = 5,
    IOSAPIC = 6,
    LocalSAPIC = 7,
    PlatformInterruptSource = 8,
    // Processor Local x2APIC
    PLx2A = 9,
    Localx2APICNMI = 0xA,
    MultiProcessorWakeup = 0x10,
    // Core Programmable Interrupt Controller
    COREPIC = 0x11,
    // Legacy IO Programmable Interrupt Controller
    LIOPIC = 0x12,
    // HyperTransport programmable Interrupt Controller
    HTPIC = 0x13,
    // Extend IO Programmable Interrupt Controller
    EIOPIC = 0x14,
    // MSI Programmable Interrupt Controller
    MSIPIC = 0x15,
    // Bridge IO Programmable Interrupt Controller
    BIOPIC = 0x16,
    // Low Pin Count Programmable Interrupt Controller
    LPCPIC = 0x17,
}

#[allow(dead_code)]
pub mod structures {
    // MPS INTI Flags
    // Polarity: 2 bits (Polarity of APIC I/O: 00 ?, 01 Active high, 10 Reserved, 11 Active Low)
    // Trigger mode: 2 bits (Trigger mode of APIC I/O: 00 ?, 01 Edge-triggered, 10 Reserved, 11 Level-triggered)
    // Reserved: 12 bits

    // Local APIC Flags
    // Enabled: 1 bit (If this bit is set processor is ready for use, if bit is clear and OC bit is set this processor can be enabled)
    // Online Capable: 1 bit (If bit is set processor can be enabled)
    // Reserved: 30 bits

    #[repr(C, packed)]
    pub struct ProcessorLocalAPIC {
        stype: u8,
        length: u8,
        acpi_processor_uid: u8,
        apic_id: u8,
        // Local APIC Flags
        flags: u32,
    }

    #[repr(C, packed)]
    pub struct IOAPIC {
        stype: u8,
        length: u8,
        id: u8,
        _reserved: u8,
        address: u32,
        // Global System Interrupt Base
        // Indicates where the IOAPIC interrupts start
        gsib: u32,
    }

    #[repr(C, packed)]
    pub struct InterruptSourceOverride {
        stype: u8,
        length: u8,
        bus: u8,
        // Bus relative interrupt source
        source: u8,
        // Global System Interrupt
        // This is the GSI that the bus-relative interrupt source will signal.
        gsi: u32,
        // MPS INTI Flags
        flags: u16,
    }

    #[repr(C, packed)]
    pub struct NMISource {
        stype: u8,
        length: u8,
        // MPS INTI FLAGS
        flags: u16,
        // Global System Interrupt
        // This is the GSI that the NMI will signal
        gsi: u32,
    }

    #[repr(C, packed)]
    pub struct LocalAPICNMI {
        stype: u8,
        length: u8,
        acpi_processor_uid: u8,
        // MPS INTI FLAGS
        flags: u16,
        // Local APIC Interrupt Input LINT number to which the NMI is connected
        local_apic_lint_num: u8,
    }

    #[repr(C, packed)]
    pub struct LAAO {
        stype: u8,
        length: u8,
        _reserved: u16,
        // Physical address of local APIC
        local_apic_address: u64,
    }

    #[repr(C, packed)]
    pub struct IOSAPIC {
        stype: u8,
        length: u8,
        io_apic_id: u8,
        _reserved: u8,
        gsib: u32,
        io_sapic_address: u64,
    }

    #[repr(C, packed)]
    pub struct LocalSAPIC {
        stype: u8,
        length: u8,
        acpi_processor_id: u8,
        local_sapic_id: u8,
        local_sapic_edi: u8,
        _reserved: [u8; 3],
        // Local APIC Flags
        flags: u32,
        acpi_processor_uid: u32,
        // ACPI_PROCESSOR_UID_STRING
        // >= 1 and null terminated
    }

    #[repr(C, packed)]
    pub struct PlatformInterruptSource {
        stype: u8,
        length: u8,
        // MPS INTI FLAGS
        flags: u16,
        // 1 PMI, 2 INIT, 3 Correct Platform Error Interrupt
        interrupt_type: u8,
        processor_id: u8,
        processor_eid: u8,
        // Value that OSPM must use to program vector feild of the IO SAPIC redirection table entry
        // for those with PMI interrupt type
        io_sapic_vector: u8,
        gsi: u32,
        // Platform Interrupt Source flags
        // CPEI Processor Override: 1 bit (When set indicates retrievel of error information is
        // allowed)
        // reserved: 31 bits
        pisf: u32,
    }

    #[repr(C, packed)]
    pub struct PLx2A {
        stype: u8,
        length: u8,
        _reserved: u16,
        x2apic_id: u32,
        // Local APIC Flags
        flags: u32,
        acpi_processor_uid: u32,
    }

    #[repr(C, packed)]
    pub struct Localx2APICNMI {
        stype: u8,
        length: u8,
        // MPS INTI Flags
        flags: u16,
        acpi_processor_uid: u32,
        local_x2apic_lint_num: u8,
        _reserved: [u8; 3],
    }

    #[repr(C, packed)]
    pub struct MultiprocessorWakeup {
        stype: u8,
        length: u8,
        mailbox_version: u16,
        _reserved: u32,
        mailbox_address: u64,
    }

    // TODO Create structs for the rest
}
