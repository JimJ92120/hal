// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=111
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct TCNT1L;

impl Register for TCNT1L {
    const ADDRESS: Address = 0x84 as Address;
}
