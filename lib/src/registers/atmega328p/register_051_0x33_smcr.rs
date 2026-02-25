// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=35
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SMCR;

impl Register for SMCR {
    const ADDRESS: Address = (IO_OFFSET + 0x33) as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum SMCRBitField {
    SE = 0,
    SM0 = 1,
    SM1 = 2,
    SM2 = 3,
    // 4
    // 5
    // 6
    // 7
}
