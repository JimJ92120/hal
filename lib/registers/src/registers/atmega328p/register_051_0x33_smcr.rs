// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=35
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SMCR;

impl Register<u8> for SMCR {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x33) as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum SMCRBitField {
    SE = 0,
    SM0 = 1,
    SM1 = 2,
    SM2 = 3,
    // 4
    // 5
    // 6
    // 7
}
