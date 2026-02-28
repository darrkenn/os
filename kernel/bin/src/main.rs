#![no_std]
#![no_main]

pub mod init;

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
    fb_println,
    time::{datetime, delay},
};
// Import is actually used
#[allow(unused_imports)]
use lib::panic;

use crate::init::init;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init(boot_info);

    fb_println!("Hello");

    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();

    loop {}
}
