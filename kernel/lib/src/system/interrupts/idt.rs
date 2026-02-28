use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{
    fb_println,
    output::framebuffer::{frame_buffer::FrameBufferColour, print::change_colour},
};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(crate::system::interrupts::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    change_colour(FrameBufferColour::Red);
    fb_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    change_colour(FrameBufferColour::White);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    change_colour(FrameBufferColour::Red);
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
