// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=111
use crate::globals::Register;

#[derive(Debug)]
pub struct TCNT1H;

impl Register<u8> for TCNT1H {
    const ADDRESS: *mut u8 = 0x85 as *mut u8;
}
