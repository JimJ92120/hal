// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=73
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct PINC;

impl Register<u8> for PINC {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x06) as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum PINCBitField {
    AC0 = 0,
    AC1 = 1,
    AC2 = 2,
    AC3 = 3,
    AC4 = 4,
    AC5 = 5,
    // 6
    // 7
}
