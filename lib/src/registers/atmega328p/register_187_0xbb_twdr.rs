// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=200
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct TWDR;

impl Register for TWDR {
    const ADDRESS: Address = 0xBB as Address;
}
