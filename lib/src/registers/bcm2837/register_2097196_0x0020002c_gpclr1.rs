// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=72
use crate::globals::{ Register, Bit, Address };
use super::MMIO_BASE_OFFSET;

#[derive(Debug)]
pub struct GPCLR1;

impl Register for GPCLR1 {
    const ADDRESS: Address = (MMIO_BASE_OFFSET + 0x0020002C) as Address;
}

// GPIO 32-53
impl GPCLR1 {
    pub const PIN_32: Bit = Bit::Zero;
    pub const PIN_33: Bit = Bit::One;
    pub const PIN_34: Bit = Bit::Two;
    pub const PIN_35: Bit = Bit::Three;
    pub const PIN_36: Bit = Bit::Four;
    pub const PIN_37: Bit = Bit::Five;
    pub const PIN_38: Bit = Bit::Six;
    pub const PIN_39: Bit = Bit::Seven;
    pub const PIN_40: Bit = Bit::Eight;
    pub const PIN_41: Bit = Bit::Nine;
    pub const PIN_42: Bit = Bit::Ten;
    pub const PIN_43: Bit = Bit::Eleven;
    pub const PIN_44: Bit = Bit::Twelve;
    pub const PIN_45: Bit = Bit::Thirteen;
    pub const PIN_46: Bit = Bit::Fourteen;
    pub const PIN_47: Bit = Bit::Fifteen;
    pub const PIN_48: Bit = Bit::Sixteen;
    pub const PIN_49: Bit = Bit::Seventeen;
    pub const PIN_50: Bit = Bit::Eighteen;
    pub const PIN_51: Bit = Bit::Nineteen;
    pub const PIN_52: Bit = Bit::Twenty;
    pub const PIN_53: Bit = Bit::TwentyOne;
    // pub const PIN_54: Bit = Bit::TwentyTwo;
    // pub const PIN_55: Bit = Bit::TwentyThree;
    // pub const PIN_56: Bit = Bit::TwentyFour;
    // pub const PIN_57: Bit = Bit::TwentyFive;
    // pub const PIN_58: Bit = Bit::TwentySix;
    // pub const PIN_59: Bit = Bit::TwentySeven;
    // pub const PIN_60: Bit = Bit::TwentyEight;
    // pub const PIN_61: Bit = Bit::TwentyNine;
    // pub const PIN_62: Bit = Bit::Thirty;
    // pub const PIN_63: Bit = Bit::ThirtyOne;
}
