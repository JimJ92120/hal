// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TCCR0B;

impl Register for TCCR0B {
    const ADDRESS: Address = (IO_OFFSET + 0x25) as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TCCR0BBitField {
    CS00 = 0,
    CS01 = 1,
    CS02 = 2,
    WGM02 = 3,
    // 4
    // 5
    FOC0B = 6,
    FOC0A = 7,
}
