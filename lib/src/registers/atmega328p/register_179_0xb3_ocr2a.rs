// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=131
use crate::globals::Register;

#[derive(Debug)]
pub struct OCR2A;

impl Register<u8> for OCR2A {
    const ADDRESS: *mut u8 = 0xB3 as *mut u8;
}
