mod handlers;
pub mod tables;

pub fn init() {
    tables::gdt::init();
    tables::idt::init();
}
