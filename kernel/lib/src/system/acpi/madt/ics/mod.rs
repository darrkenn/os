pub mod structures;

#[repr(u8)]
pub enum ICSType {
    ProcessorLocalAPIC = 0,
    IOAPIC = 1,
    InterruptSourceOverride = 2,
    NMISource = 3,
    LocalAPICNMI = 4,
    LocalAPICAddressOverride = 5,
    IOSAPIC = 6,
    LocalSAPIC = 7,
    PlatfomInterruptSources = 8,
    ProcessorLocalx2APIC = 9,
    Localx2APIC = 0xA,
    GICC = 0xB,
    GICD = 0xC,
    GICMSIFrame = 0xD,
    GICR = 0xE,
    ITS = 0xF,
    MultiprocessorWakeup = 0x10,
    LIOPIC = 0x12,
    HTPIC = 0x13,
    EIOPIC = 0x14,
    MSIPIC = 0x15,
    BIOPIC = 0x16,
    LPCPIC = 0x17,
}
