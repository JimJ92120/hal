// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=108
use crate::globals::Register;

#[derive(Debug)]
pub struct TCCR1A;

impl Register<u8> for TCCR1A {
    const ADDRESS: *mut u8 = 0x80 as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TCCR1ABitField {
    WGM10 = 0,
    WGM11 = 1,
    // 2
    // 3
    COM1B0 = 4,
    COM1B1 = 5,
    COM1A0 = 6,
    COM1A1 = 7,
}
