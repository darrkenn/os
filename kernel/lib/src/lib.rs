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

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::system::exit::qemu::{QemuExitCode, exit_qemu};

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failure);
    loop {}
}

pub fn lib_test() {
    assert_eq!("a", "adsdad")
}
