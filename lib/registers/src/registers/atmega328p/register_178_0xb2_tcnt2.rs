// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=131
use crate::globals::Register;

#[derive(Debug)]
pub struct TCNT2;

impl Register<u8> for TCNT2 {
    const ADDRESS: *mut u8 = 0xB2 as *mut u8;
}
