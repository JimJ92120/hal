// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=57
use crate::globals::Register;

#[derive(Debug)]
pub struct PCMSK2;

impl Register<u8> for PCMSK2 {
    const ADDRESS: *mut u8 = 0x6D as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum PCMSK2BitField {
    PCINT16 = 0,
    PCINT17 = 1,
    PCINT18 = 2,
    PCINT19 = 3,
    PCINT20 = 4,
    PCINT21 = 5,
    PCINT22 = 6,
    PCINT23 = 7,
}
