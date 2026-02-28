use bootloader_api::BootInfo;
use lib::{output::framebuffer::FRAME_BUFFER_WRITER, system};

pub fn init(boot_info: &'static mut BootInfo) {
    system::interrupts::init();

    // Setup framebuffer
    let frame_buffer = boot_info.framebuffer.as_mut().unwrap();
    let frame_buffer_info = frame_buffer.info().clone();
    FRAME_BUFFER_WRITER
        .lock()
        .set(frame_buffer.buffer_mut(), frame_buffer_info);
    // Clear framebuffer
    FRAME_BUFFER_WRITER
        .lock()
        .clear()
        .expect("Unable to clear framebuffer");
}
