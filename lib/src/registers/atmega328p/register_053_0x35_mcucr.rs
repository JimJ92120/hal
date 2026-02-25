// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=38
// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=52
// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=72
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct MCUCR;

impl Register for MCUCR {
    const ADDRESS: Address = (IO_OFFSET + 0x35) as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum MCUCRBitField {
    IVCE = 0,
    IVSEL = 1,
    // 2
    // 3
    PUD = 4,
    BODSE = 5,
    BODS = 6,
    // 7
}
