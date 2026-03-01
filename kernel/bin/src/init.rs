use bootloader_api::BootInfo;
use lib::{
    fb_println,
    output::framebuffer::FRAME_BUFFER_WRITER,
    system::{
        interrupts::tables::rsdp_table::RsdpTable,
        physical_memory::{PHYSICAL_MEMORY_OFFSET, convert_physical_to_virtual_addr},
    },
    utils::deadlock::lock_mutex,
};

pub fn init(boot_info: &'static mut BootInfo) {
    // Setup framebuffer
    let frame_buffer = boot_info.framebuffer.as_mut().unwrap();
    let frame_buffer_info = frame_buffer.info().clone();
    lock_mutex(&FRAME_BUFFER_WRITER).set(frame_buffer.buffer_mut(), frame_buffer_info);
    // Clear framebuffer
    lock_mutex(&FRAME_BUFFER_WRITER)
        .clear()
        .expect("Unable to clear framebuffer");

    // Get the address of the rsdp table
    let rsdp_addr = match boot_info.rsdp_addr.into_option() {
        Some(addr) => addr,
        None => {
            panic!("No rsdp_addr found");
        }
    };

    // Get phyiscal memory offset
    match boot_info.physical_memory_offset.into_option() {
        Some(pmo) => lock_mutex(&PHYSICAL_MEMORY_OFFSET).set(pmo),
        None => {
            panic!("No phyiscal memory offset found, ensure enabled in boot config")
        }
    };

    let rsdp_table = RsdpTable::new(convert_physical_to_virtual_addr(rsdp_addr));
    fb_println!("{:#?}", rsdp_table);
}
