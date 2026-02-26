// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=112
use crate::globals::Register;

#[derive(Debug)]
pub struct ICR1H;

impl Register<u8> for ICR1H {
    const ADDRESS: *mut u8 = 0x87 as *mut u8;
}
