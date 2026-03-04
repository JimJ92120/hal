use lib_registers::{
    globals::{ Register },
    bcm::{
        GPFSEL0, GPFSEL0BitField,
        GPFSEL1, GPFSEL1BitField,
        GPFSEL2, GPFSEL2BitField,
        GPSET0, GPSET0BitField,
        GPCLR0, GPCLR0BitField
    }
};

mod pin;

pub use pin::Pin;

#[derive(Debug)]
pub struct GPIO;

impl GPIO {
    pub fn set_output(pin: Pin) {
        let bit = Self::get_fsel_bit_field(&pin);

        match pin {
            Pin::Zero | Pin::One
                => panic!("Special pins not implemented."),
            Pin::Two | Pin::Three | Pin::Four | Pin::Five | Pin::Six | Pin::Seven | Pin::Eight | Pin::Nine
                => GPFSEL0::or(1 << bit),
            Pin::Ten | Pin::Eleven | Pin::Twelve | Pin::Thirteen | Pin::Fourteen | Pin::Fifteen | Pin::Sixteen
            | Pin::Seventeen | Pin::Eighteen | Pin::Nineteen
                => GPFSEL1::or(1 << bit),
            Pin::Twenty | Pin::TwentyOne | Pin::TwentyTwo | Pin::TwentyThree | Pin::TwentyFour | Pin::TwentyFive
            | Pin::TwentySix | Pin::TwentySeven
                => GPFSEL2::or(1 << bit),
        }
    }

    pub fn set_input(pin: Pin) {
        let bit = Self::get_fsel_bit_field(&pin);

        match pin {
            Pin::Zero | Pin::One
                => panic!("Special pins not implemented."),
            Pin::Two | Pin::Three | Pin::Four | Pin::Five | Pin::Six | Pin::Seven | Pin::Eight | Pin::Nine
                => GPFSEL0::and(!(1 << bit)),
            Pin::Ten | Pin::Eleven | Pin::Twelve | Pin::Thirteen | Pin::Fourteen | Pin::Fifteen | Pin::Sixteen
            | Pin::Seventeen | Pin::Eighteen | Pin::Nineteen
                => GPFSEL1::and(!(1 << bit)),
            Pin::Twenty | Pin::TwentyOne | Pin::TwentyTwo | Pin::TwentyThree | Pin::TwentyFour | Pin::TwentyFive
            | Pin::TwentySix | Pin::TwentySeven
                => GPFSEL2::and(!(1 << bit)),
        };
    }

    pub fn set_high(pin: Pin) {
        let bit = Self::get_set_bit_field(&pin);

        GPSET0::or(1 << bit);
    }

    pub fn set_low(pin: Pin) {
        let bit = Self::get_clear_bit_field(&pin);

        GPCLR0::or(1 << bit);
    }

    fn get_fsel_bit_field(pin: &Pin) -> u32 {
        match pin {
            Pin::Zero | Pin::One    => panic!("Special pins not implemented."),
            Pin::Two                => GPFSEL0BitField::FSEL2 as u32,
            Pin::Three              => GPFSEL0BitField::FSEL3 as u32,
            Pin::Four               => GPFSEL0BitField::FSEL4 as u32,
            Pin::Five               => GPFSEL0BitField::FSEL5 as u32,
            Pin::Six                => GPFSEL0BitField::FSEL6 as u32,
            Pin::Seven              => GPFSEL0BitField::FSEL7 as u32,
            Pin::Eight              => GPFSEL0BitField::FSEL8 as u32,
            Pin::Nine               => GPFSEL0BitField::FSEL9 as u32,
            Pin::Ten                => GPFSEL1BitField::FSEL10 as u32,
            Pin::Eleven             => GPFSEL1BitField::FSEL11 as u32,
            Pin::Twelve             => GPFSEL1BitField::FSEL12 as u32,
            Pin::Thirteen           => GPFSEL1BitField::FSEL13 as u32,
            Pin::Fourteen           => GPFSEL1BitField::FSEL14 as u32,
            Pin::Fifteen            => GPFSEL1BitField::FSEL15 as u32,
            Pin::Sixteen            => GPFSEL1BitField::FSEL16 as u32,
            Pin::Seventeen          => GPFSEL1BitField::FSEL17 as u32,
            Pin::Eighteen           => GPFSEL1BitField::FSEL18 as u32,
            Pin::Nineteen           => GPFSEL1BitField::FSEL19 as u32,
            Pin::Twenty             => GPFSEL2BitField::FSEL20 as u32,
            Pin::TwentyOne          => GPFSEL2BitField::FSEL21 as u32,
            Pin::TwentyTwo          => GPFSEL2BitField::FSEL22 as u32,
            Pin::TwentyThree        => GPFSEL2BitField::FSEL23 as u32,
            Pin::TwentyFour         => GPFSEL2BitField::FSEL24 as u32,
            Pin::TwentyFive         => GPFSEL2BitField::FSEL25 as u32,
            Pin::TwentySix          => GPFSEL2BitField::FSEL26 as u32,
            Pin::TwentySeven        => GPFSEL2BitField::FSEL27 as u32,
        }
    }

    fn get_set_bit_field(pin: &Pin) -> u32 {
        match pin {
            Pin::Zero | Pin::One    => panic!("Special pins not implemented."),
            Pin::Two                => GPCLR0BitField::PIN02 as u32,
            Pin::Three              => GPCLR0BitField::PIN03 as u32,
            Pin::Four               => GPCLR0BitField::PIN04 as u32,
            Pin::Five               => GPCLR0BitField::PIN05 as u32,
            Pin::Six                => GPCLR0BitField::PIN06 as u32,
            Pin::Seven              => GPCLR0BitField::PIN07 as u32,
            Pin::Eight              => GPCLR0BitField::PIN08 as u32,
            Pin::Nine               => GPCLR0BitField::PIN09 as u32,
            Pin::Ten                => GPCLR0BitField::PIN10 as u32,
            Pin::Eleven             => GPCLR0BitField::PIN11 as u32,
            Pin::Twelve             => GPCLR0BitField::PIN12 as u32,
            Pin::Thirteen           => GPCLR0BitField::PIN13 as u32,
            Pin::Fourteen           => GPCLR0BitField::PIN14 as u32,
            Pin::Fifteen            => GPCLR0BitField::PIN15 as u32,
            Pin::Sixteen            => GPCLR0BitField::PIN16 as u32,
            Pin::Seventeen          => GPCLR0BitField::PIN17 as u32,
            Pin::Eighteen           => GPCLR0BitField::PIN18 as u32,
            Pin::Nineteen           => GPCLR0BitField::PIN19 as u32,
            Pin::Twenty             => GPCLR0BitField::PIN20 as u32,
            Pin::TwentyOne          => GPCLR0BitField::PIN21 as u32,
            Pin::TwentyTwo          => GPCLR0BitField::PIN22 as u32,
            Pin::TwentyThree        => GPCLR0BitField::PIN23 as u32,
            Pin::TwentyFour         => GPCLR0BitField::PIN24 as u32,
            Pin::TwentyFive         => GPCLR0BitField::PIN25 as u32,
            Pin::TwentySix          => GPCLR0BitField::PIN26 as u32,
            Pin::TwentySeven        => GPCLR0BitField::PIN27 as u32,
        }
    }

    fn get_clear_bit_field(pin: &Pin) -> u32 {
        match pin {
            Pin::Zero | Pin::One    => panic!("Special pins not implemented."),
            Pin::Two                => GPCLR0BitField::PIN02 as u32,
            Pin::Three              => GPCLR0BitField::PIN03 as u32,
            Pin::Four               => GPCLR0BitField::PIN04 as u32,
            Pin::Five               => GPCLR0BitField::PIN05 as u32,
            Pin::Six                => GPCLR0BitField::PIN06 as u32,
            Pin::Seven              => GPCLR0BitField::PIN07 as u32,
            Pin::Eight              => GPCLR0BitField::PIN08 as u32,
            Pin::Nine               => GPCLR0BitField::PIN09 as u32,
            Pin::Ten                => GPCLR0BitField::PIN10 as u32,
            Pin::Eleven             => GPCLR0BitField::PIN11 as u32,
            Pin::Twelve             => GPCLR0BitField::PIN12 as u32,
            Pin::Thirteen           => GPCLR0BitField::PIN13 as u32,
            Pin::Fourteen           => GPCLR0BitField::PIN14 as u32,
            Pin::Fifteen            => GPCLR0BitField::PIN15 as u32,
            Pin::Sixteen            => GPCLR0BitField::PIN16 as u32,
            Pin::Seventeen          => GPCLR0BitField::PIN17 as u32,
            Pin::Eighteen           => GPCLR0BitField::PIN18 as u32,
            Pin::Nineteen           => GPCLR0BitField::PIN19 as u32,
            Pin::Twenty             => GPCLR0BitField::PIN20 as u32,
            Pin::TwentyOne          => GPCLR0BitField::PIN21 as u32,
            Pin::TwentyTwo          => GPCLR0BitField::PIN22 as u32,
            Pin::TwentyThree        => GPCLR0BitField::PIN23 as u32,
            Pin::TwentyFour         => GPCLR0BitField::PIN24 as u32,
            Pin::TwentyFive         => GPCLR0BitField::PIN25 as u32,
            Pin::TwentySix          => GPCLR0BitField::PIN26 as u32,
            Pin::TwentySeven        => GPCLR0BitField::PIN27 as u32,
        }
    }
}