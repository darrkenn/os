use x86_64::instructions::interrupts;

use crate::{
    output::framebuffer::{FRAME_BUFFER_WRITER, frame_buffer::FrameBufferColour},
    utils::deadlock::lock_mutex,
};

pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    interrupts::without_interrupts(|| {
        lock_mutex(&FRAME_BUFFER_WRITER)
            .write_fmt(args)
            .expect("Printing to framebuffer failed")
    })
}

pub fn change_colour(colour: FrameBufferColour) {
    lock_mutex(&FRAME_BUFFER_WRITER).change_colour(colour);
}

#[macro_export]
macro_rules! fb_print{
    ($($arg:tt)*) => {
        $crate::output::framebuffer::print::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! fb_println{
    () => ($crate::fb_print!("\n"));
    ($fmt:expr) => ($crate::fb_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::fb_print!(
        concat!($fmt, "\n"), $($arg)*));
}
