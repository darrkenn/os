use x86_64::instructions::port::Port;

pub struct CMOS {
    address: Port<u8>,
    data: Port<u8>,
}

#[repr(u8)]
pub enum CmosRegister {
    Century = 0x32,
    Year = 0x09,
    Month = 0x08,
    Day = 0x07,
    Hour = 0x04,
    Minute = 0x02,
    Second = 0x00,
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
    pub fn read(&mut self, reg: CmosRegister) -> u8 {
        unsafe {
            self.address.write(reg as u8);
        }
        unsafe { self.data.read() }
    }
    pub fn read_binary(&mut self, reg: CmosRegister) -> u8 {
        unsafe {
            self.address.write(reg as u8);
        }
        let value = unsafe { self.data.read() };
        crate::convert::bcd_to_binary(value)
    }
}
