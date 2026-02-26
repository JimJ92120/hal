// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=111
use crate::globals::Register;

#[derive(Debug)]
pub struct TCNT1L;

impl Register<u8> for TCNT1L {
    const ADDRESS: *mut u8 = 0x84 as *mut u8;
}
