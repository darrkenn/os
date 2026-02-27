#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// This macro creates a test with the name of the function provided and calls it
#[macro_export]
macro_rules! call_test {
    ($function:ident) => {
        #[test_case]
        fn $function() {
            $function();
        }
    };
}

use bootloader_api::{BootInfo, entry_point};
use lib::{
    serial_println,
    system::{self, exit},
    time::datetime,
};
// Import is actually used
#[allow(unused_imports)]
use lib::panic;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    serial_println!("Hello world{}", "!");

    let datetime = datetime::DateTime::new();

    serial_println!("Second: {}", datetime.second);
    serial_println!("Minute: {}", datetime.minute);
    serial_println!("Hour: {}", datetime.hour);
    serial_println!("Day: {}", datetime.day);
    serial_println!("Month: {}", datetime.month);
    serial_println!("Year: {}", datetime.year);
    serial_println!("Century: {}", datetime.century);

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
mod tests {
    use lib::lib_test;

    call_test!(lib_test);
}

#[allow(dead_code)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    system::exit::qemu::exit_qemu(exit::qemu::QemuExitCode::Success);
}
