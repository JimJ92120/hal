// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=127
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct TCCR2A;

impl Register for TCCR2A {
    const ADDRESS: Address = 0xB0 as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TCCR2ABitField {
    WGM20 = 0,
    WGM21 = 1,
    // 2
    // 3
    COM2B0 = 4,
    COM2B1 = 5,
    COM2A0 = 6,
    COM2A1 = 7,
}
