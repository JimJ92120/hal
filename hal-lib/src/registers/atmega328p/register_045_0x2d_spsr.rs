// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=141
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPSR;

impl Register for SPSR {
    const ADDRESS: Address = (IO_OFFSET + 0x2D) as Address;
}

impl SPSR {
    pub const SPI2X: Bit = Bit::Zero;
    // 1
    // 2
    // 3
    // 4
    // 5
    pub const WCOL: Bit = Bit::Six;
    pub const SPIF: Bit = Bit::Seven;
}
