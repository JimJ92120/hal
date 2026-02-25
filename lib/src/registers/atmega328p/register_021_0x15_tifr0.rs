// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TIFR0;

impl Register for TIFR0 {
    const ADDRESS: Address = (IO_OFFSET + 0x15) as Address;
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
