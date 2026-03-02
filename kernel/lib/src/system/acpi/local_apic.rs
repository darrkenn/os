// Spurious-Interrupt Vector Register
const SVR_OFFSET: u64 = 0xF0;

// Divide Configuration Register
const DVR_OFFSET: u64 = 0x3E0;

// Task Priority Register
//const TPR_OFFSET: u8 = 0x80;

fn get_ptr(local_apic_addr: u64, offset: u64) -> *mut u32 {
    (local_apic_addr + offset) as *mut u32
}

pub unsafe fn enable_apic(local_apic_addr: u64) {
    unsafe {
        // Sets bit 8 of the SVR, therefore enabling apic
        let svr_ptr = get_ptr(local_apic_addr, SVR_OFFSET);
        let mut svr = core::ptr::read_volatile(svr_ptr);
        svr |= 1 << 0;
        core::ptr::write_volatile(svr_ptr, svr);
    }
}

pub unsafe fn set_dvr(local_apic_addr: u64) {
    unsafe {
        // Divide configuration register only looks at bit 0,1 and 3.
        // Setting bit values:
        // Bit 0 = 0
        // Bit 1 = 0
        // Bit 3 = 1
        // Will result in a divide value of 32
        let dvr_ptr = get_ptr(local_apic_addr, DVR_OFFSET);
        let mut dvr = core::ptr::read_volatile(dvr_ptr);
        dvr &= !1 << 0;
        dvr &= !1 << 1;
        dvr |= 1 << 3;
        core::ptr::write_volatile(dvr_ptr, dvr);
    }
}

