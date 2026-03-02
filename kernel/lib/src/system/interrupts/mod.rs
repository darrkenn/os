mod handlers;
pub mod pic;
pub mod tables;

pub fn init() {
    unsafe { pic::disable_8259_pic() };
    tables::gdt::init();
    tables::idt::init();
    x86_64::instructions::interrupts::enable();
}
