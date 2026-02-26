use crate::GLOBAL_CMOS;

static CENTURY_REGISTER: u8 = 0x32;
static YEAR_REGISTER: u8 = 0x09;
static MONTH_REGISTER: u8 = 0x08;
static DAY_REGISTER: u8 = 0x07;
static HOUR_REGISTER: u8 = 0x04;
static MINUTE_REGISTER: u8 = 0x02;
static SECOND_REGISTER: u8 = 0x00;

#[allow(dead_code)]
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
            century: cmos.read_binary(CENTURY_REGISTER),
            year: cmos.read_binary(YEAR_REGISTER),
            month: cmos.read_binary(MONTH_REGISTER),
            day: cmos.read_binary(DAY_REGISTER),
            hour: cmos.read_binary(HOUR_REGISTER),
            minute: cmos.read_binary(MINUTE_REGISTER),
            second: cmos.read_binary(SECOND_REGISTER),
        }
    }
    pub fn update(&mut self, dtf: DateTimeField) {
        let mut cmos = GLOBAL_CMOS.lock();
        match dtf {
            DateTimeField::Century => self.century = cmos.read_binary(CENTURY_REGISTER),
            DateTimeField::Year => self.year = cmos.read_binary(YEAR_REGISTER),
            DateTimeField::Month => self.month = cmos.read_binary(MONTH_REGISTER),
            DateTimeField::Day => self.day = cmos.read_binary(DAY_REGISTER),
            DateTimeField::Hour => self.hour = cmos.read_binary(HOUR_REGISTER),
            DateTimeField::Minute => self.hour = cmos.read_binary(MINUTE_REGISTER),
            DateTimeField::Second => self.second = cmos.read_binary(SECOND_REGISTER),
        }
    }
    pub fn to_str<'a>() -> &'a str {
        ""
    }
}
