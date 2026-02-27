// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=57
use crate::globals::Register;

#[derive(Debug)]
pub struct PCMSK0;

impl Register<u8> for PCMSK0 {
    const ADDRESS: *mut u8 = 0x6B as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum PCMSK0BitField {
    PCINT0 = 0,
    PCINT1 = 1,
    PCINT2 = 2,
    PCINT3 = 3,
    PCINT4 = 4,
    PCINT5 = 5,
    PCINT6 = 6,
    PCINT7 = 7,
}
