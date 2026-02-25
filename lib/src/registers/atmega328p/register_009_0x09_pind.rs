// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=73
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct PIND;

impl Register for PIND {
    const ADDRESS: Address = (IO_OFFSET + 0x09) as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum PINDBitField {
    PD0 = 0,
    PD1 = 1,
    PD2 = 2,
    PD3 = 3,
    PD4 = 4,
    PD5 = 5,
    PD6 = 6,
    PD7 = 7,
}
