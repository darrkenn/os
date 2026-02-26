#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};

use vga::WRITER;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    unreachable!()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
