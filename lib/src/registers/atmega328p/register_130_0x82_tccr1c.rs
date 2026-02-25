// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct TCCR1C;

impl Register for TCCR1C {
    const ADDRESS: Address = 0x82 as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TCCR1CBitField {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    FOC1B = 6,
    FOC1A = 7,
}
