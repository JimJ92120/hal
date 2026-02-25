// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TCCR0A;

impl Register for TCCR0A {
    const ADDRESS: Address = (IO_OFFSET + 0x24) as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TCCR0ABitField {
    WGM00 = 0,
    WGM01 = 1,
    // 2
    // 3
    COM0B0 = 4,
    COM0B1 = 5,
    COM0A0 = 6,
    COM0A1 = 7,
}
