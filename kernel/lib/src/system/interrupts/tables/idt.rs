use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{
    fb_print, fb_println,
    output::framebuffer::{frame_buffer::FrameBufferColour, print::change_colour},
    system::interrupts::handlers::faults::{breakpoint_handler, double_fault_handler},
};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(crate::system::interrupts::tables::gdt::DOUBLE_FAULT_IST_INDEX);
        };
        idt
    };
}

pub fn init() {
    IDT.load();
}
