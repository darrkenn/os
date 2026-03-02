#[allow(dead_code)]
use crate::system::acpi::local_apic::{CCR_OFFSET, ICR_OFFSET, LVT_TR_OFFSET, get_ptr};

pub enum TimerMode {
    OneShot,
    Periodic,
}

pub unsafe fn setup_timer(local_apic_addr: u64, timer_mode: TimerMode) {
    unsafe {
        let lvt_timer_ptr = get_ptr(local_apic_addr, LVT_TR_OFFSET);
        set_timer_mode(lvt_timer_ptr, timer_mode);
        unmask_timer(lvt_timer_ptr);
        start_timer(get_ptr(local_apic_addr, ICR_OFFSET));
    }
}

unsafe fn set_timer_mode(lvt_timer_ptr: *mut u32, timer_mode: TimerMode) {
    unsafe {
        let mut lvt_timer = core::ptr::read_volatile(lvt_timer_ptr);
        match timer_mode {
            TimerMode::OneShot => {
                lvt_timer &= !(1 << 17);
                lvt_timer &= !(1 << 16);
            }
            TimerMode::Periodic => {
                lvt_timer &= !(1 << 17);
                lvt_timer |= (1 << 16);
            }
        }
        core::ptr::write_volatile(lvt_timer_ptr, lvt_timer);
    }
}

unsafe fn unmask_timer(lvt_timer_ptr: *mut u32) {
    unsafe {
        let mut lvt_timer = core::ptr::read_volatile(lvt_timer_ptr);
        lvt_timer &= !(1 << 15);
        core::ptr::write_volatile(lvt_timer_ptr, lvt_timer);
    }
}

unsafe fn start_timer(icr_ptr: *mut u32) {
    unsafe {
        core::ptr::write_volatile(icr_ptr, 100_000_000);
    }
}
