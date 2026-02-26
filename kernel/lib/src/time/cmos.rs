use x86_64::instructions::port::Port;

pub struct CMOS {
    address: Port<u8>,
    data: Port<u8>,
}

static ADDRESS_PORT: u8 = 0x70;
static DATA_PORT: u8 = 0x71;

impl CMOS {
    pub unsafe fn new() -> CMOS {
        CMOS {
            address: Port::new(ADDRESS_PORT as u16),
            data: Port::new(DATA_PORT as u16),
        }
    }
    pub fn read(&mut self, reg: u8) -> u8 {
        unsafe {
            self.address.write(reg);
        }
        unsafe { self.data.read() }
    }
    pub fn read_binary(&mut self, reg: u8) -> u8 {
        unsafe {
            self.address.write(reg);
        }
        let value = unsafe { self.data.read() };
        crate::convert::bcd_to_binary(value)
    }
    /*
    pub fn read(&mut self, reg: u8) -> u8 {

        unsafe { self.address.write(reg) };
        unsafe { self.data.read() };
    }
    */
}
