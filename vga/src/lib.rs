#![no_std]

mod colour;
mod writer;

use lazy_static::lazy_static;

use crate::writer::Writer;

// VGA memory address
pub static VGA_BUFFER: u32 = 0xb8000;
pub static BUFFER_HEIGHT: usize = 25;
pub static BUFFER_WIDTH: usize = 80;

lazy_static! {
    pub static ref WRITER: spin::Mutex<Writer> = spin::Mutex::new(Writer::new());
}
