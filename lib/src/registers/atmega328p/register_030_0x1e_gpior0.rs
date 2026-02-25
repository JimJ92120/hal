// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=23
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct GPIOR0;

impl Register for GPIOR0 {
    const ADDRESS: Address = (IO_OFFSET + 0x1E) as Address;
}
