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

use lib::{
    system::{self, exit},
    time::datetime,
};
// Import is actually used
#[allow(unused_imports)]
use lib::panic;
use vga::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    let datetime = datetime::DateTime::new();

    println!("Second: {}", datetime.second);
    println!("Minute: {}", datetime.minute);
    println!("Hour: {}", datetime.hour);
    println!("Day: {}", datetime.day);
    println!("Month: {}", datetime.month);
    println!("Year: {}", datetime.year);
    println!("Century: {}", datetime.century);

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
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    system::exit::qemu::exit_qemu(exit::qemu::QemuExitCode::Success);
}
