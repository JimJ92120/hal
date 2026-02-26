// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=201
use crate::globals::Register;

#[derive(Debug)]
pub struct TWAR;

impl Register<u8> for TWAR {
    const ADDRESS: *mut u8 = 0xBA as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TWARBitField {
    TWGCE = 0,
    TWA0 = 1,
    TWA1 = 2,
    TWA2 = 3,
    TWA3 = 4,
    TWA4 = 5,
    TWA5 = 6,
    TWA6 = 7,
}
