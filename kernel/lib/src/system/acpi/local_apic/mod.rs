use crate::system::acpi::local_apic::timer::{TimerMode, setup_timer};

pub mod timer;
// Spurious-Interrupt Vector Register
const SVR_OFFSET: u64 = 0xF0;

// Divide Configuration Register
const DCR_OFFSET: u64 = 0x3E0;

// Task Priority Register
//const TPR_OFFSET: u8 = 0x80;

// LVT Timer Register
const LVT_TR_OFFSET: u64 = 0x320;

// Initial Count Register
const ICR_OFFSET: u64 = 0x380;

// Current Count Register
//const CCR_OFFSET: u64 = 0x390;

// EOI register
const EOI_OFFSET: u64 = 0xB0;

static mut EOI_PTR: *mut u32 = 0 as *mut u32;

pub fn signal_end_of_interrupt() {
    unsafe { core::ptr::write_volatile(EOI_PTR, 1) };
}

fn get_ptr(local_apic_addr: u64, offset: u64) -> *mut u32 {
    (local_apic_addr + offset) as *mut u32
}

pub unsafe fn init(local_apic_addr: u64, timer_mode: TimerMode) {
    unsafe {
        enable_apic(local_apic_addr);
        set_dcr(local_apic_addr);
        EOI_PTR = get_ptr(local_apic_addr, EOI_OFFSET);
        setup_timer(local_apic_addr, timer_mode);
    }
}

unsafe fn enable_apic(local_apic_addr: u64) {
    unsafe {
        // Sets bit 8 of the SVR, therefore enabling apic
        let svr_ptr = get_ptr(local_apic_addr, SVR_OFFSET);
        let mut svr = core::ptr::read_volatile(svr_ptr);
        svr |= 1 << 7;
        core::ptr::write_volatile(svr_ptr, svr);
    }
}

unsafe fn set_dcr(local_apic_addr: u64) {
    unsafe {
        // Divide configuration register only looks at bit 0,1 and 3.
        // Setting bit values:
        // Bit 0 = 0
        // Bit 1 = 0
        // Bit 3 = 1
        // Will result in a divide value of 32
        let dcr_ptr = get_ptr(local_apic_addr, DCR_OFFSET);
        let mut dcr = core::ptr::read_volatile(dcr_ptr);
        dcr &= !(1 << 0);
        dcr &= !(1 << 1);
        dcr |= 1 << 3;
        core::ptr::write_volatile(dcr_ptr, dcr);
    }
}
