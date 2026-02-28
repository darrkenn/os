#![no_std]

pub mod convert;
pub mod output;
pub mod system;
pub mod time;

use crate::time::cmos::CMOS;
use core::panic::PanicInfo;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_CMOS: spin::Mutex<CMOS> = spin::Mutex::new(unsafe { CMOS::new() });
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    loop {}
}
