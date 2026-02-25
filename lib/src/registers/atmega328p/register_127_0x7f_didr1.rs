// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=204
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct DIDR1;

impl Register for DIDR1 {
    const ADDRESS: Address = 0x7F as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum DIDR1BitField {
    AIN0D = 0,
    AIN1D = 1,
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
