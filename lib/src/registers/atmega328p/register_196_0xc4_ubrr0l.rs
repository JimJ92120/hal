// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=162
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct UBRR0L;

impl Register for UBRR0L {
    const ADDRESS: Address = 0xC4 as Address;
}
