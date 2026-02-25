// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=111
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct TCNT1H;

impl Register for TCNT1H {
    const ADDRESS: Address = 0x85 as Address;
}
