// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=130
use crate::globals::Register;

#[derive(Debug)]
pub struct TCCR2B;

impl Register<u8> for TCCR2B {
    const ADDRESS: *mut u8 = 0xB1 as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TCCR2BBitField {
    CS20 = 0,
    CS21 = 1,
    CS22 = 2,
    WGM22 = 3,
    // 4
    // 5
    FOC2B = 6,
    FOC2A = 7,
}
