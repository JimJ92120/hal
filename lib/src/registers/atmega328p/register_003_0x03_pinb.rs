// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=72
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct PINB;

impl Register for PINB {
    const ADDRESS: Address = (IO_OFFSET + 0x03) as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum PINBBitField {
    PB0 = 0,
    PB1 = 1,
    PB2 = 2,
    PB3 = 3,
    PB4 = 4,
    PB5 = 5,
    // 6
    // 7
}
