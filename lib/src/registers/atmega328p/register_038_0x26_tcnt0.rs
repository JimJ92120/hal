// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TCNT0;

impl Register for TCNT0 {
    const ADDRESS: Address = (IO_OFFSET + 0x26) as Address;
}
