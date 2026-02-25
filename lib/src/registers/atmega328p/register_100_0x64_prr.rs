// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=36
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct PRR;

impl Register for PRR {
    const ADDRESS: Address = 0x64 as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum PRRBitField {
    PRADC = 0,
    PRUSAR0 = 1,
    PRSPI = 2,
    PRTIM1 = 3,
    // 4
    PRTIM0 = 5,
    PRTIM2 = 6,
    PRTWI = 7,
}
