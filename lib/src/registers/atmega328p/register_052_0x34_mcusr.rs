// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=46
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct MCUSR;

impl Register<u8> for MCUSR {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x34) as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum MCUSRBitField {
    PORF = 0,
    EXTRF = 1,
    BORF = 2,
    WDRF = 3,
    // 4
    // 5
    // 6
    // 7
}
