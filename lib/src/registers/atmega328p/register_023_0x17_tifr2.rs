// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=132
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TIFR2;

impl Register for TIFR2 {
    const ADDRESS: Address = (IO_OFFSET + 0x17) as Address;
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
