pub trait Ics {
    fn structure_type(&self) -> u8;
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
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

    #[repr(C, packed)]
    pub struct ICSTypeLength {
        pub stype: u8,
        pub length: u8,
    }

    // MPS INTI Flags
    // Polarity: 2 bits (Polarity of APIC I/O: 00 ?, 01 Active high, 10 Reserved, 11 Active Low)
    // Trigger mode: 2 bits (Trigger mode of APIC I/O: 00 ?, 01 Edge-triggered, 10 Reserved, 11 Level-triggered)
    // Reserved: 12 bits

    // Local APIC Flags
    // Enabled: 1 bit (If this bit is set processor is ready for use, if bit is clear and OC bit is set this processor can be enabled)
    // Online Capable: 1 bit (If bit is set processor can be enabled)
    // Reserved: 30 bits

    use crate::system::acpi::madt::ics::Ics;

    #[repr(C, packed)]
    pub struct ProcessorLocalAPIC {
        type_length: ICSTypeLength,
        acpi_processor_uid: u8,
        apic_id: u8,
        // Local APIC Flags
        flags: u32,
    }
    impl Ics for ProcessorLocalAPIC {
        fn structure_type(&self) -> u8 {
            self.type_length.stype
        }
    }

    #[repr(C, packed)]
    pub struct IOAPIC {
        type_length: ICSTypeLength,
        stype: u8,
        length: u8,
        id: u8,
        _reserved: u8,
        address: u32,
        // Global System Interrupt Base
        // Indicates where the IOAPIC interrupts start
        gsib: u32,
    }
    impl Ics for IOAPIC {
        fn structure_type(&self) -> u8 {
            self.type_length.stype
        }
    }
    impl IOAPIC {
        pub fn address(&self) -> u32 {
            self.address
        }
    }

    #[repr(C, packed)]
    pub struct InterruptSourceOverride {
        type_length: ICSTypeLength,
        bus: u8,
        // Bus relative interrupt source
        source: u8,
        // Global System Interrupt
        // This is the GSI that the bus-relative interrupt source will signal.
        gsi: u32,
        // MPS INTI Flags
        flags: u16,
    }
    impl Ics for InterruptSourceOverride {
        fn structure_type(&self) -> u8 {
            self.type_length.stype
        }
    }

    #[repr(C, packed)]
    pub struct NMISource {
        type_length: ICSTypeLength,
        // MPS INTI FLAGS
        flags: u16,
        // Global System Interrupt
        // This is the GSI that the NMI will signal
        gsi: u32,
    }
    impl Ics for NMISource {
        fn structure_type(&self) -> u8 {
            self.type_length.stype
        }
    }

    #[repr(C, packed)]
    pub struct LocalAPICNMI {
        type_length: ICSTypeLength,
        acpi_processor_uid: u8,
        // MPS INTI FLAGS
        flags: u16,
        // Local APIC Interrupt Input LINT number to which the NMI is connected
        local_apic_lint_num: u8,
    }
    impl Ics for LocalAPICNMI {
        fn structure_type(&self) -> u8 {
            self.type_length.stype
        }
    }

    #[repr(C, packed)]
    pub struct LAAO {
        type_length: ICSTypeLength,
        _reserved: u16,
        // Physical address of local APIC
        local_apic_address: u64,
    }
    impl Ics for LAAO {
        fn structure_type(&self) -> u8 {
            self.type_length.stype
        }
    }

    #[repr(C, packed)]
    pub struct IOSAPIC {
        type_length: ICSTypeLength,
        io_apic_id: u8,
        _reserved: u8,
        gsib: u32,
        io_sapic_address: u64,
    }
    impl Ics for IOSAPIC {
        fn structure_type(&self) -> u8 {
            self.type_length.stype
        }
    }

    #[repr(C, packed)]
    pub struct LocalSAPIC {
        type_length: ICSTypeLength,
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
    impl Ics for LocalSAPIC {
        fn structure_type(&self) -> u8 {
            self.type_length.stype
        }
    }

    #[repr(C, packed)]
    pub struct PlatformInterruptSource {
        type_length: ICSTypeLength,
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
    impl Ics for PlatformInterruptSource {
        fn structure_type(&self) -> u8 {
            self.type_length.stype
        }
    }

    #[repr(C, packed)]
    pub struct PLx2A {
        type_length: ICSTypeLength,
        _reserved: u16,
        x2apic_id: u32,
        // Local APIC Flags
        flags: u32,
        acpi_processor_uid: u32,
    }
    impl Ics for PLx2A {
        fn structure_type(&self) -> u8 {
            self.type_length.stype
        }
    }

    #[repr(C, packed)]
    pub struct Localx2APICNMI {
        type_length: ICSTypeLength,
        // MPS INTI Flags
        flags: u16,
        acpi_processor_uid: u32,
        local_x2apic_lint_num: u8,
        _reserved: [u8; 3],
    }
    impl Ics for Localx2APICNMI {
        fn structure_type(&self) -> u8 {
            self.type_length.stype
        }
    }

    #[repr(C, packed)]
    pub struct MultiprocessorWakeup {
        type_length: ICSTypeLength,
        mailbox_version: u16,
        _reserved: u32,
        mailbox_address: u64,
    }
    impl Ics for MultiprocessorWakeup {
        fn structure_type(&self) -> u8 {
            self.type_length.stype
        }
    }

    // TODO Create structs for the rest
}
