// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=20
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct EECR;

impl Register<u8> for EECR {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x1F) as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum EECRBitField {
    EERE = 0,
    EEPE = 1,
    EEMPE = 2,
    EERIE = 3,
    EEPM0 = 4,
    EEPM1 = 5,
    // 6
    // 7
}
