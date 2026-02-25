// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=218
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct ADCSRA;

impl Register for ADCSRA {
    const ADDRESS: Address = 0x7A as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum ADCSRABitField {
    ADPS0 = 0,
    ADPS1 = 1,
    ADPS2 = 2,
    ADIE = 3,
    ADIF = 4,
    ADATE = 5,
    ADSC = 6,
    ADEN = 7,
}
