pub mod frame_buffer;
pub mod print;
use lazy_static::lazy_static;
use spin::mutex::Mutex;

use crate::output::framebuffer::frame_buffer::FrameBufferWriter;

lazy_static! {
    pub static ref FRAME_BUFFER_WRITER: Mutex<FrameBufferWriter> = {
        let frame_buffer_writer = FrameBufferWriter::new();
        Mutex::new(frame_buffer_writer)
    };
}
