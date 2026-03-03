use x86_64::structures::idt::InterruptStackFrame;

use crate::{
    fb_println,
    output::framebuffer::{frame_buffer::FrameBufferColour, print::change_colour},
    serial_println,
    system::acpi,
};

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    change_colour(FrameBufferColour::Red);
    fb_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    change_colour(FrameBufferColour::White);
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    change_colour(FrameBufferColour::Red);
    panic!(
        "EXCEPTION: DOUBLE FAULT ({})\n{:#?}",
        error_code, stack_frame
    );
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    serial_println!("timer");
    acpi::local_apic::signal_end_of_interrupt();
}
