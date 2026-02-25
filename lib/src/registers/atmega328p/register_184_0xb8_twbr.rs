// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=198
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct TWBR;

impl Register for TWBR {
    const ADDRESS: Address = 0xB8 as Address;
}
