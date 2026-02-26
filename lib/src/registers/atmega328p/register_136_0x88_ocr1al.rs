// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=111
use crate::globals::Register;

#[derive(Debug)]
pub struct OCR1AL;

impl Register<u8> for OCR1AL {
    const ADDRESS: *mut u8 = 0x88 as *mut u8;
}
