// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=110
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TCCR1B;

impl Register for TCCR1B {
    const ADDRESS: Address = 0x81 as Address;
}

impl TCCR1B {
    pub const CS10: Bit = Bit::Zero;
    pub const CS11: Bit = Bit::One;
    pub const CS12: Bit = Bit::Two;
    pub const WGM12: Bit = Bit::Three;
    pub const WGM13: Bit = Bit::Four;
    // 5
    pub const ICES1: Bit = Bit::Six;
    pub const ICNC1: Bit = Bit::Seven;
}
