#![no_std]
#![feature(abi_x86_interrupt)]

pub mod output;
pub mod system;
pub mod time;
pub mod utils;

use crate::time::cmos::CMOS;
use core::panic::PanicInfo;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_CMOS: spin::Mutex<CMOS> = spin::Mutex::new(unsafe { CMOS::new() });
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    fb_println!("{}", info);
    serial_println!("{}", info);
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
