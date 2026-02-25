// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=203
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct ACSR;

impl Register for ACSR {
    const ADDRESS: Address = (IO_OFFSET + 0x30) as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum ACSRBitField {
    ACIS0 = 0,
    ACIS1 = 1,
    ACIC = 2,
    ACIE = 3,
    ACI = 4,
    ACO = 5,
    ACBG = 6,
    ACD = 7,
}
