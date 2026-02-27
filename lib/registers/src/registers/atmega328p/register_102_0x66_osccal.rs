// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=32
use crate::globals::Register;

#[derive(Debug)]
pub struct OSCCAL;

impl Register<u8> for OSCCAL {
    const ADDRESS: *mut u8 = 0x66 as *mut u8;
}

