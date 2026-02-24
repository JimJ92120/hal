use crate::{
    globals::Bit,
    registers::atmega328p::{ PORTB, PORTC, PORTD }
};

#[derive(Debug)]
pub enum Pin {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
}

impl Pin {
    pub fn bit(&self) -> Bit {
        match self {
            Pin::Zero       => PORTD::PD0,
            Pin::One        => PORTD::PD1,
            Pin::Two        => PORTD::PD2,
            Pin::Three      => PORTD::PD3,
            Pin::Four       => PORTD::PD4,
            Pin::Five       => PORTD::PD5,
            Pin::Six        => PORTD::PD6,
            Pin::Seven      => PORTD::PD7,
            Pin::Eight      => PORTB::PB0,
            Pin::Nine       => PORTB::PB1,
            Pin::Ten        => PORTB::PB2,
            Pin::Eleven     => PORTB::PB3,
            Pin::Twelve     => PORTB::PB4,
            Pin::Thirteen   => PORTB::PB5,
            Pin::Fourteen   => PORTC::AC0,
            Pin::Fifteen    => PORTC::AC1,
            Pin::Sixteen    => PORTC::AC2,
            Pin::Seventeen  => PORTC::AC3,
            Pin::Eighteen   => PORTC::AC4,
            Pin::Nineteen   => PORTC::AC5,
        }
    }
}
