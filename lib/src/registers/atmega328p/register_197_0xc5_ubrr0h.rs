// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=162
use crate::globals::Register;

#[derive(Debug)]
pub struct UBRR0H;

impl Register<u8> for UBRR0H {
    const ADDRESS: *mut u8 = 0xC5 as *mut u8;
}
