use crate::system::acpi::local_apic::{ICR_OFFSET, LVT_TR_OFFSET, get_ptr};

pub enum TimerMode {
    OneShot,
    Periodic,
}

pub unsafe fn setup_timer(local_apic_addr: u64, timer_mode: TimerMode) {
    unsafe {
        let lvt_timer_ptr = get_ptr(local_apic_addr, LVT_TR_OFFSET);

        set_lvt_ivn(lvt_timer_ptr);
        set_timer_mode(lvt_timer_ptr, timer_mode);
        unmask_timer(lvt_timer_ptr);
        // TODO: Properly configure interrupt speed
        start_timer(get_ptr(local_apic_addr, ICR_OFFSET));
    }
}

unsafe fn set_timer_mode(lvt_timer_ptr: *mut u32, timer_mode: TimerMode) {
    unsafe {
        let mut lvt_timer = core::ptr::read_volatile(lvt_timer_ptr);
        match timer_mode {
            TimerMode::OneShot => {
                // One shot mode counts down the ICR to 0 and fires an interrupt
                // Requiring another ICR value to be set before timer begins again
                // Setting both bit 18 and 17 to 0 will set one shot mode
                lvt_timer &= !(1 << 18);
                lvt_timer &= !(1 << 17);
            }
            TimerMode::Periodic => {
                // Periodic mode counts down the CCR to 0, fires an interrupt then resets CCR to ICR.
                // Setting bit 18 to 0 and 17 to 1 will set periodic mode
                lvt_timer &= !(1 << 18);
                lvt_timer |= 1 << 17;
            }
        }
        core::ptr::write_volatile(lvt_timer_ptr, lvt_timer);
    }
}

unsafe fn unmask_timer(lvt_timer_ptr: *mut u32) {
    unsafe {
        // Set interrupt mask bit to 0
        let mut lvt_timer = core::ptr::read_volatile(lvt_timer_ptr);
        lvt_timer &= !(1 << 16);
        core::ptr::write_volatile(lvt_timer_ptr, lvt_timer);
    }
}

unsafe fn set_lvt_ivn(lvt_timer_ptr: *mut u32) {
    unsafe {
        // Sets ivn to 32 (timer)
        core::ptr::write_volatile(lvt_timer_ptr, 32);
    }
}

unsafe fn start_timer(icr_ptr: *mut u32) {
    unsafe {
        core::ptr::write_volatile(icr_ptr, 500_000);
    }
}
