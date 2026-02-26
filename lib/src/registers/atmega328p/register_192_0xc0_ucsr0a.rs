// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=159
use crate::globals::Register;

#[derive(Debug)]
pub struct UCSR0A;

impl Register<u8> for UCSR0A {
    const ADDRESS: *mut u8 = 0xC0 as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum UCSR0ABitField {
    MPCM0 = 0,
    U2X0 = 1,
    UPE0 = 2,
    DOR0 = 3,
    FE0 = 4,
    UDRE0 = 5,
    TXC0 = 6,
    RXC0 = 7,
}
