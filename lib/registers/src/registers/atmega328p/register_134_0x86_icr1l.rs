// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=112
use crate::globals::Register;

#[derive(Debug)]
pub struct ICR1L;

impl Register<u8> for ICR1L {
    const ADDRESS: *mut u8 = 0x86 as *mut u8;
}
