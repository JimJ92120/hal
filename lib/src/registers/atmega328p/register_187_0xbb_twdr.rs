// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=200
use crate::globals::Register;

#[derive(Debug)]
pub struct TWDR;

impl Register<u8> for TWDR {
    const ADDRESS: *mut u8 = 0xBB as *mut u8;
}
