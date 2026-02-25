// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=46
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct MCUSR;

impl Register for MCUSR {
    const ADDRESS: Address = (IO_OFFSET + 0x34) as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum MCUSRBitField {
    PORF = 0,
    EXTRF = 1,
    BORF = 2,
    WDRF = 3,
    // 4
    // 5
    // 6
    // 7
}
