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

use bootloader_api::{BootInfo, config::Mapping, entry_point};
use lib::{fb_println, time::delay};
// Import is actually used
#[allow(unused_imports)]
use lib::panic;

use crate::init::init;

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init(boot_info);

    fb_println!("Hello");

    delay::from_secs(10);
    fb_println!("Delay 10 secs later");

    loop {}
}
