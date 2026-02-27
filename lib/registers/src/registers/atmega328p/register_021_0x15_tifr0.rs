// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TIFR0;

impl Register<u8> for TIFR0 {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x15) as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TIFR0BitField {
    TOV0 = 0,
    OCF0A = 1,
    OCF0B = 2,
    // 3
    // 4
    // 5
    // 6
    // 7
}
