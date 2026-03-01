use bootloader_api::BootInfo;
use lib::{output::framebuffer::FRAME_BUFFER_WRITER, system, utils::deadlock::lock_mutex};

pub fn init(boot_info: &'static mut BootInfo) {
    // Setup framebuffer
    let frame_buffer = boot_info.framebuffer.as_mut().unwrap();
    let frame_buffer_info = frame_buffer.info().clone();
    lock_mutex(&FRAME_BUFFER_WRITER).set(frame_buffer.buffer_mut(), frame_buffer_info);
    // Clear framebuffer
    lock_mutex(&FRAME_BUFFER_WRITER)
        .clear()
        .expect("Unable to clear framebuffer");
    system::interrupts::init();
}
