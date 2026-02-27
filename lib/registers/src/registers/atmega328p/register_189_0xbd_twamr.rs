// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=201
use crate::globals::Register;

#[derive(Debug)]
pub struct TWAMR;

impl Register<u8> for TWAMR {
    const ADDRESS: *mut u8 = 0xBD as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TWAMRBitField {
    // 0
    TWAM0 = 1,
    TWAM1 = 2,
    TWAM2 = 3,
    TWAM3 = 4,
    TWAM4 = 5,
    TWAM5 = 6,
    TWAM6 = 7,
}
