// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=142
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPDR;

impl Register for SPDR {
    const ADDRESS: Address = (IO_OFFSET + 0x2E) as Address;
}
