#![no_std]

// VGA memory address
static VGA_BUFFER: u32 = 0xb8000;

pub fn write_to_screen(text: &[u8]) {
    let vga_buffer = VGA_BUFFER as *mut u8;
    for (i, &byte) in text.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}
