use crate::output::framebuffer::FRAME_BUFFER_WRITER;

pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    FRAME_BUFFER_WRITER
        .lock()
        .write_fmt(args)
        .expect("Printing to framebuffer failed");
}

#[macro_export]
macro_rules! fb_print{
    ($($arg:tt)*) => {
        $crate::output::framebuffer::print::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! fb_println{
    () => ($crate::fb_print!("\n"));
    ($fmt:expr) => ($crate::fb_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::fb_print!(
        concat!($fmt, "\n"), $($arg)*));
}
