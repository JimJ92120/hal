// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=140
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPCR;

impl Register for SPCR {
    const ADDRESS: Address = (IO_OFFSET + 0x2C) as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum SPCRBitField {
    SPR0 = 0,
    SPR1 = 1,
    CPHA = 2,
    CPOL = 3,
    MSTR = 4,
    DORD = 5,
    SPE = 6,
    SPIE = 7,
}
