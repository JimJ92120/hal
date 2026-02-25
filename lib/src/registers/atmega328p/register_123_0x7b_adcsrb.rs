// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=220
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct ADCSRB;

impl Register for ADCSRB {
    const ADDRESS: Address = 0x7B as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum ADCSRBBitField {
    ADTS0 = 0,
    ADTS1 = 1,
    ADTS2 = 2,
    // 3
    // 4
    // 5
    ACME = 6,
    // 7
}
