// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=47
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct WDTCSR;

impl Register for WDTCSR {
    const ADDRESS: Address = 0x60 as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum WDTCSRBitField {
    WDP0 = 0,
    WDP1 = 1,
    WDP2 = 2,
    WDE = 3,
    WDCE = 4,
    WDP3 = 5,
    WDIE = 6,
    WDIF = 7,
}
