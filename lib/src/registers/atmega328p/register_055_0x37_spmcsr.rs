// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=239
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPMCSR;

impl Register for SPMCSR {
    const ADDRESS: Address = (IO_OFFSET + 0x37) as Address;
}

impl SPMCSR {
    pub const SELFPRGN: Bit = Bit::Zero;
    pub const PGERS: Bit = Bit::One;
    pub const PGWRT: Bit = Bit::Two;
    pub const BLBSET: Bit = Bit::Three;
    pub const RWWSRE: Bit = Bit::Four;
    // 5
    pub const RWWSB: Bit = Bit::Six;
    pub const SPMIE: Bit = Bit::Seven;
}
