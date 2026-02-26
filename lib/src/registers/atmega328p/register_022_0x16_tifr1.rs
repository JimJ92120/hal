// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=113
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TIFR1;

impl Register<u8> for TIFR1 {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x16) as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TIFR1BitField {
    TOV1 = 0,
    OCF1A = 1,
    OCF1B = 2,
    // 3
    // 4
    // 5
    // 6
    // 7
}
