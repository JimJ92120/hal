// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=219
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct ADCH;

impl Register for ADCH {
    const ADDRESS: Address = 0x79 as Address;
}
