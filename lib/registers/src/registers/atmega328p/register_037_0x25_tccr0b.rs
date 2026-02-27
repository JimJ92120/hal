// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TCCR0B;

impl Register<u8> for TCCR0B {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x25) as *mut u8;
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
