// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=13
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPL;

impl Register for SPL {
    const ADDRESS: Address = (IO_OFFSET + 0x3D) as Address;
}

impl SPL {
    pub const SP0: Bit = Bit::Zero;
    pub const SP1: Bit = Bit::One;
    pub const SP2: Bit = Bit::Two;
    pub const SP3: Bit = Bit::Three;
    pub const SP4: Bit = Bit::Four;
    pub const SP5: Bit = Bit::Five;
    pub const SP6: Bit = Bit::Six;
    pub const SP7: Bit = Bit::Seven;
}
