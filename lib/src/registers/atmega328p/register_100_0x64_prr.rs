// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=36
use crate::globals::Register;

#[derive(Debug)]
pub struct PRR;

impl Register<u8> for PRR {
    const ADDRESS: *mut u8 = 0x64 as *mut u8;
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
