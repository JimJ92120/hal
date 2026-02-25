// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=162
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct UBRR0H;

impl Register for UBRR0H {
    const ADDRESS: Address = 0xC5 as Address;
}
