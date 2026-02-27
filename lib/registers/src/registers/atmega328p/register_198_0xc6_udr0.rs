// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=159
use crate::globals::Register;

#[derive(Debug)]
pub struct UDR0;

impl Register<u8> for UDR0 {
    const ADDRESS: *mut u8 = 0xC6 as *mut u8;
}
