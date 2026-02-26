#![no_std]

#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::{nop, port::Port};
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32)
    }
    loop {
        nop();
    }
}
