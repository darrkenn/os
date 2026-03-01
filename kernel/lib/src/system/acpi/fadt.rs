use crate::system::acpi::sdt::SdtHeader;

pub enum AddressType {
    Base,
    Extended,
}

#[repr(C, packed)]
struct GenericAddressStructure {
    address_space: u8,
    bit_width: u8,
    bit_offset: u8,
    access_size: u8,
    address: u64,
}

#[repr(C, packed)]
pub struct FADT {
    h: SdtHeader,
    firmware_ctrl_addr: u32,
    dsdt_addr: u32,
    _reserved: u8,
    preferred_pm_profile: u8,
    sci_int: u16,
    smi_cmd: u32,
    acpi_enable: u8,
    acpi_disable: u8,
    s4bios_req: u8,
    pstate_ctrl: u8,
    pm1a_event_block: u32,
    pm1b_event_block: u32,
    pm1a_ctrl_block: u32,
    pm1b_ctrl_block: u32,
    pm2_ctrl_block: u32,
    pm_timer_ctrl_block: u32,
    gpe0_block: u32,
    gpe1_block: u32,
    pm1_event_length: u8,
    pm1_ctrl_length: u8,
    pm2_ctrl_length: u8,
    pm_timer_length: u8,
    gpe0_block_length: u8,
    gpe1_block_length: u8,
    gpe1_base: u8,
    cstate_ctrl: u8,
    worst_c2state_latency: u16,
    worst_c3state_latency: u16,
    flush_size: u16,
    flush_stride: u16,
    duty_offset: u8,
    duty_width: u8,
    day_alarm: u8,
    month_alarm: u8,
    century: u8,
    iapc_boot_arch_flags: u16,
    _reserved2: u8,
    flags: u32,
    reset_register: GenericAddressStructure,
    reset_value: u8,
    arm_boot_arch_flags: u16,
    fadt_minor_version: u8,
    x_firmware_ctrl_addr: u64,
    x_dsdt_addr: u64,
    x_pm1a_event_block: GenericAddressStructure,
    x_pm1b_event_block: GenericAddressStructure,
    x_pm1a_ctrl_block: GenericAddressStructure,
    x_pm1b_ctrl_block: GenericAddressStructure,
    x_pm2_ctrl_block: GenericAddressStructure,
    x_pm_timer_block: GenericAddressStructure,
    x_gpe0_block: GenericAddressStructure,
    x_gpe1_block: GenericAddressStructure,
    sleep_control_register: GenericAddressStructure,
    sleep_status_register: GenericAddressStructure,
    hypervisor_vendor_identity: GenericAddressStructure,
}

impl FADT {
    pub fn new(xsdt_addr: u64) -> Self {
        let ptr = xsdt_addr as *const FADT;
        unsafe { ptr.read_unaligned() }
    }
    pub fn header(&self) -> SdtHeader {
        self.h
    }
    // If x_firmware_ctrl_addr field is non-zero then ignore firmware_ctrl_addr
    pub fn which_firmware_ctrl(&self) -> AddressType {
        if self.x_firmware_ctrl_addr != 0 {
            AddressType::Extended
        } else {
            AddressType::Base
        }
    }
    pub fn firmware_ctrl_addr(&self) -> u32 {
        self.firmware_ctrl_addr
    }
    pub fn extended_firmware_ctrl_addr(&self) -> u64 {
        self.x_firmware_ctrl_addr
    }

    // If x_dsdt_addr field is non-zero then ignore dsdt_addr
    pub fn which_dsdt(&self) -> AddressType {
        if self.x_dsdt_addr != 0 {
            AddressType::Extended
        } else {
            AddressType::Base
        }
    }
    pub fn dsdt_addr(&self) -> u32 {
        self.dsdt_addr
    }
    pub fn extended_dsdt_addr(&self) -> u64 {
        self.x_dsdt_addr
    }
}
