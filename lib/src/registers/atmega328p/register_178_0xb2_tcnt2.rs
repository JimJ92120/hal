// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=131
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct TCNT2;

impl Register for TCNT2 {
    const ADDRESS: Address = 0xB2 as Address;
}
