// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=219
use crate::globals::Register;

#[derive(Debug)]
pub struct ADCL;

impl Register<u8> for ADCL {
    const ADDRESS: *mut u8 = 0x78 as *mut u8;
}
