// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=159
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct UDR0;

impl Register for UDR0 {
    const ADDRESS: Address = 0xC6 as Address;
}
