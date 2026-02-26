// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=57
use crate::globals::Register;

#[derive(Debug)]
pub struct PCMSK1;

impl Register<u8> for PCMSK1 {
    const ADDRESS: *mut u8 = 0x6C as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum PCMSK1BitField {
    PCINT8 = 0,
    PCINT9 = 1,
    PCINT10 = 2,
    PCINT11 = 3,
    PCINT12 = 4,
    PCINT13 = 5,
    PCINT14 = 6,
    // 7
}
