#![no_std]
#![no_main]

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

use bootloader_api::{BootInfo, entry_point};
use lib::{fb_println, output::framebuffer::FRAME_BUFFER_WRITER, serial_println, time::datetime};
// Import is actually used
#[allow(unused_imports)]
use lib::panic;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let frame_buffer = boot_info.framebuffer.as_mut().unwrap();
    let frame_buffer_info = frame_buffer.info().clone();

    FRAME_BUFFER_WRITER
        .lock()
        .set(frame_buffer.buffer_mut(), frame_buffer_info);

    FRAME_BUFFER_WRITER.lock().clear();
    let datetime = datetime::DateTime::new();

    fb_println!(
        "Second: {}\nMinute: {}\nHour: {}\nDay: {}\nMonth: {}\nYear: {}\nCentury: {}\n",
        datetime.second,
        datetime.minute,
        datetime.hour,
        datetime.day,
        datetime.month,
        datetime.year,
        datetime.century
    );

    serial_println!("Hello world{}", "!");

    loop {}
}
