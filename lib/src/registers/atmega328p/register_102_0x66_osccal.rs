// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=32
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct OSCCAL;

impl Register for OSCCAL {
    const ADDRESS: Address = 0x66 as Address;
}

