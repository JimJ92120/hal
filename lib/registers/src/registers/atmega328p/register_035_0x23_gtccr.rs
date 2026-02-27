// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=115
// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=134
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct GTCCR;

impl Register<u8> for GTCCR {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x23) as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum GTCCRBitField {
    PSRSYNC = 0,
    PSRASY = 1,
    // 2
    // 3
    // 4
    // 5
    // 6
    TSM = 7,
}
