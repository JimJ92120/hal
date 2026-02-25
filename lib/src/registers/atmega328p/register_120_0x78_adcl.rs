// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=219
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct ADCL;

impl Register for ADCL {
    const ADDRESS: Address = 0x78 as Address;
}
