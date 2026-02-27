// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=140
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPCR;

impl Register<u8> for SPCR {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x2C) as *mut u8;
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
