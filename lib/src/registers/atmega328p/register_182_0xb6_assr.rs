// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=133
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct ASSR;

impl Register for ASSR {
    const ADDRESS: Address = 0xB6 as Address;
}

impl ASSR {
    pub const TCR2BUB: Bit = Bit::Zero;
    pub const TCR2AUB: Bit = Bit::One;
    pub const OCR2BUB: Bit = Bit::Two;
    pub const OCR2AUB: Bit = Bit::Three;
    pub const TCN2UB: Bit = Bit::Four;
    pub const AS2: Bit = Bit::Five;
    pub const EXCLK: Bit = Bit::Six;
    // 7
}
