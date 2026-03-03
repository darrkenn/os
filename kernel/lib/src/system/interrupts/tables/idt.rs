use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::system::interrupts::handlers::{
    breakpoint_handler, double_fault_handler, timer_interrupt_handler,
};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(0);
        };
        idt[32].set_handler_fn(timer_interrupt_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}
