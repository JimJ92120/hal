// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=73
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct PINC;

impl Register for PINC {
    const ADDRESS: Address = (IO_OFFSET + 0x06) as Address;
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
