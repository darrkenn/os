#![no_std]
#![no_main]

pub mod init;

use bootloader_api::{BootInfo, config::Mapping, entry_point};
use lib::fb_println;
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

    fb_println!("Im here!");
    loop {
        x86_64::instructions::hlt();
    }
}
