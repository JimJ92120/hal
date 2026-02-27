// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=217
use crate::globals::Register;

#[derive(Debug)]
pub struct ADMUX;

impl Register<u8> for ADMUX {
    const ADDRESS: *mut u8 = 0x7C as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum ADMUXBitField {
    MUX0 = 0,
    MUX1 = 1,
    MUX2 = 2,
    MUX3 = 3,
    // 4
    ADLAR = 5,
    REFS0 = 6,
    REFS1 = 7,
}
