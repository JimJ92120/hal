// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=132
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TIFR2;

impl Register<u8> for TIFR2 {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x17) as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TIFR2BitField {
    TOV2 = 0,
    OCF2A = 1,
    OCF2B = 2,
    // 3
    // 4
    // 5
    // 6
    // 7
}
