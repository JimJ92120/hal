// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=113
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TIFR1;

impl Register for TIFR1 {
    const ADDRESS: Address = (IO_OFFSET + 0x16) as Address;
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
