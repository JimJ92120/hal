// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=204
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct DIDR1;

impl Register for DIDR1 {
    const ADDRESS: Address = 0x7F as Address;
}

impl DIDR1 {
    pub const AIN0D: Bit = Bit::Zero;
    pub const AIN1D: Bit = Bit::One;
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
