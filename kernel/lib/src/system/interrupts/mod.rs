use crate::fb_println;

mod handlers;
pub mod pic;
pub mod tables;

pub fn init() {
    unsafe { pic::disable_8259_pic() };
    fb_println!("8259 PIC disabled");
    tables::gdt::init();
    tables::idt::init();
    fb_println!("IDT, GDT started");
    x86_64::instructions::interrupts::enable();
    fb_println!("Interrupts enabled");
}
