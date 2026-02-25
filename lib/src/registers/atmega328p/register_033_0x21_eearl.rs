// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=20
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct EEARL;

impl Register for EEARL {
    const ADDRESS: Address = (IO_OFFSET + 0x21) as Address;
}
