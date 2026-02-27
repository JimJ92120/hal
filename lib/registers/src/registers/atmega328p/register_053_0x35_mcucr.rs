// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=38
// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=52
// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=72
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct MCUCR;

impl Register<u8> for MCUCR {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x35) as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum MCUCRBitField {
    IVCE = 0,
    IVSEL = 1,
    // 2
    // 3
    PUD = 4,
    BODSE = 5,
    BODS = 6,
    // 7
}
