// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=72
use crate::globals::{ Register, Bit, Address };
use super::MMIO_BASE_OFFSET;

#[derive(Debug)]
pub struct GPSET0;

impl Register for GPSET0 {
    const ADDRESS: Address = (MMIO_BASE_OFFSET + 0x0020001C) as Address;
}

// GPIO 0-31
impl GPSET0 {
    pub const PIN_00: Bit = Bit::Zero;
    pub const PIN_01: Bit = Bit::One;
    pub const PIN_02: Bit = Bit::Two;
    pub const PIN_03: Bit = Bit::Three;
    pub const PIN_04: Bit = Bit::Four;
    pub const PIN_05: Bit = Bit::Five;
    pub const PIN_06: Bit = Bit::Six;
    pub const PIN_07: Bit = Bit::Seven;
    pub const PIN_08: Bit = Bit::Eight;
    pub const PIN_09: Bit = Bit::Nine;
    pub const PIN_10: Bit = Bit::Ten;
    pub const PIN_11: Bit = Bit::Eleven;
    pub const PIN_12: Bit = Bit::Twelve;
    pub const PIN_13: Bit = Bit::Thirteen;
    pub const PIN_14: Bit = Bit::Fourteen;
    pub const PIN_15: Bit = Bit::Fifteen;
    pub const PIN_16: Bit = Bit::Sixteen;
    pub const PIN_17: Bit = Bit::Seventeen;
    pub const PIN_18: Bit = Bit::Eighteen;
    pub const PIN_19: Bit = Bit::Nineteen;
    pub const PIN_20: Bit = Bit::Twenty;
    pub const PIN_21: Bit = Bit::TwentyOne;
    pub const PIN_22: Bit = Bit::TwentyTwo;
    pub const PIN_23: Bit = Bit::TwentyThree;
    pub const PIN_24: Bit = Bit::TwentyFour;
    pub const PIN_25: Bit = Bit::TwentyFive;
    pub const PIN_26: Bit = Bit::TwentySix;
    pub const PIN_27: Bit = Bit::TwentySeven;
    pub const PIN_28: Bit = Bit::TwentyEight;
    pub const PIN_29: Bit = Bit::TwentyNine;
    pub const PIN_30: Bit = Bit::Thirty;
    pub const PIN_31: Bit = Bit::ThirtyOne;
}
