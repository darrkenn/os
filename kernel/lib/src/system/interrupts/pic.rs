use x86_64::instructions::port::Port;

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[allow(dead_code)]
struct Pic {
    offset: u8,
    command: Port<u8>,
    data: Port<u8>,
}

impl Pic {
    unsafe fn write_mask(&mut self, mask: u8) {
        unsafe { self.data.write(mask) };
    }
}

pub unsafe fn disable_8259_pic() {
    unsafe {
        let mut pic1 = Pic {
            offset: PIC_1_OFFSET,
            command: Port::new(0x20),
            data: Port::new(0x21),
        };
        let mut pic2 = Pic {
            offset: PIC_2_OFFSET,
            command: Port::new(0xA0),
            data: Port::new(0xA1),
        };
        pic1.write_mask(255);
        pic2.write_mask(255);
    }
}
