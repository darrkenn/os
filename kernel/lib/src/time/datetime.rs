use core::fmt::Display;

use crate::{GLOBAL_CMOS, time::cmos::CmosRegister};

#[allow(dead_code)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DateTime {
    pub century: u8,
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

pub enum DateTimeField {
    Century,
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
}

impl DateTime {
    pub fn new() -> DateTime {
        let mut cmos = GLOBAL_CMOS.lock();
        DateTime {
            century: cmos.read_binary(CmosRegister::Century),
            year: cmos.read_binary(CmosRegister::Year),
            month: cmos.read_binary(CmosRegister::Month),
            day: cmos.read_binary(CmosRegister::Day),
            hour: cmos.read_binary(CmosRegister::Hour),
            minute: cmos.read_binary(CmosRegister::Minute),
            second: cmos.read_binary(CmosRegister::Second),
        }
    }
    pub fn reset(&mut self) {
        let mut cmos = GLOBAL_CMOS.lock();
        self.century = cmos.read_binary(CmosRegister::Century);
        self.year = cmos.read_binary(CmosRegister::Year);
        self.month = cmos.read_binary(CmosRegister::Month);
        self.day = cmos.read_binary(CmosRegister::Day);
        self.hour = cmos.read_binary(CmosRegister::Hour);
        self.hour = cmos.read_binary(CmosRegister::Minute);
        self.second = cmos.read_binary(CmosRegister::Second);
    }
    pub fn reset_field(&mut self, dtf: DateTimeField) {
        let mut cmos = GLOBAL_CMOS.lock();
        match dtf {
            DateTimeField::Century => self.century = cmos.read_binary(CmosRegister::Century),
            DateTimeField::Year => self.year = cmos.read_binary(CmosRegister::Year),
            DateTimeField::Month => self.month = cmos.read_binary(CmosRegister::Month),
            DateTimeField::Day => self.day = cmos.read_binary(CmosRegister::Day),
            DateTimeField::Hour => self.hour = cmos.read_binary(CmosRegister::Hour),
            DateTimeField::Minute => self.hour = cmos.read_binary(CmosRegister::Minute),
            DateTimeField::Second => self.second = cmos.read_binary(CmosRegister::Second),
        }
    }
    pub fn add_seconds(&mut self, seconds: u32) {
        let total_seconds = self.second as u32 + seconds;
        let total_minutes = self.minute as u32 + (total_seconds / 60);
        self.second = (total_seconds % 60) as u8;
        let total_hours = self.hour as u32 + (total_minutes / 60);
        self.minute = (total_minutes % 60) as u8;
        self.hour = (total_hours % 24) as u8;
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Second: {},Minute: {},Hour: {},Day: {}, Month: {},Year: {},Century: {}",
            self.second, self.minute, self.hour, self.day, self.month, self.year, self.century
        )
    }
}
