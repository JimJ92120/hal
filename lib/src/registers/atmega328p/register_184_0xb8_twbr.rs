// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=198
use crate::globals::Register;

#[derive(Debug)]
pub struct TWBR;

impl Register<u8> for TWBR {
    const ADDRESS: *mut u8 = 0xB8 as *mut u8;
}
