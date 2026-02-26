#![no_std]
#![no_main]

use core::panic::PanicInfo;

use kernel_lib::exit_qemu;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga::write_to_screen(b"HELLO WORLD");
    exit_qemu(kernel_lib::QemuExitCode::Success);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
