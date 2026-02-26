// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=111
use crate::globals::Register;

#[derive(Debug)]
pub struct OCR1BL;

impl Register<u8> for OCR1BL {
    const ADDRESS: *mut u8 = 0x8A as *mut u8;
}
