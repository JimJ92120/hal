// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=73
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct DDRD;

impl Register<u8> for DDRD {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x0A) as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum DDRDBitField {
    PD0 = 0,
    PD1 = 1,
    PD2 = 2,
    PD3 = 3,
    PD4 = 4,
    PD5 = 5,
    PD6 = 6,
    PD7 = 7,
}
