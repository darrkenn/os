#![no_std]

pub mod cmos;
pub mod convert;
pub mod datetime;
pub mod exit;
pub mod serial;

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use vga::WRITER;
use vga::println;
use vga::{Colour, ColourCode};

use crate::cmos::CMOS;

lazy_static! {
    pub static ref GLOBAL_CMOS: spin::Mutex<CMOS> = spin::Mutex::new(unsafe { CMOS::new() });
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    WRITER
        .lock()
        .set_colour_code(ColourCode::new(Colour::Red, Colour::Black));
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::exit::{QemuExitCode, exit_qemu};

    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failure);
    loop {}
}

pub fn lib_test() {
    assert_eq!("a", "adsdad")
}
