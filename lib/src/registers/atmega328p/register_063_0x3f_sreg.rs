// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=10
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SREG;

impl Register for SREG {
    const ADDRESS: Address = (IO_OFFSET + 0x3F) as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum SREGBitField {
    C = 0,
    Z = 1,
    N = 2,
    V = 3,
    S = 4,
    H = 5,
    T = 6,
    I = 7,
}
