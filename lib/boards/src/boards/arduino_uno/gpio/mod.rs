use lib_registers::{
    globals::{ Register },
    atmega328p::{
        DDRB, DDRBBitField,
        DDRC, DDRCBitField,
        DDRD, DDRDBitField,
    }
};

mod pin;
mod digital;
mod pwm;
mod analog;

pub use pin::Pin;
pub use digital::Digital;
pub use pwm::PWM;
pub use analog::{ Analog, AnalogSettings, AnalogMode, AnalogPrescaler };

#[derive(Debug)]
pub struct GPIO;

impl GPIO {
    pub fn set_output(pin: Pin) {
        let bit = Self::get_ddr_register_bit_field(&pin);

        match pin {
            Pin::Zero | Pin::One | Pin::Two | Pin::Three | Pin::Four | Pin::Five | Pin::Six | Pin::Seven
                // => DDRD::set_bit_mask(bit),
                => DDRD::or(1 << bit),
            Pin::Eight | Pin::Nine | Pin::Ten | Pin::Eleven | Pin::Twelve | Pin::Thirteen
                // => DDRB::set_bit_mask(bit),
                => DDRB::or(1 << bit),
            Pin::Fourteen | Pin::Fifteen | Pin::Sixteen | Pin::Seventeen | Pin::Eighteen | Pin::Nineteen
                // => DDRC::set_bit_mask(bit),
                => DDRC::or(1 << bit),
        };
    }

    pub fn set_input(pin: Pin) {
        let bit = Self::get_ddr_register_bit_field(&pin);

        match pin {
            Pin::Zero | Pin::One | Pin::Two | Pin::Three | Pin::Four | Pin::Five | Pin::Six | Pin::Seven
                // => DDRD::set_bit_mask(bit),
                => DDRD::and(!(1 << bit)),
            Pin::Eight | Pin::Nine | Pin::Ten | Pin::Eleven | Pin::Twelve | Pin::Thirteen
                // => DDRB::set_bit_mask(bit),
                => DDRB::and(!(1 << bit)),
            Pin::Fourteen | Pin::Fifteen | Pin::Sixteen | Pin::Seventeen | Pin::Eighteen | Pin::Nineteen
                // => DDRC::set_bit_mask(bit),
                => DDRC::and(!(1 << bit)),
        };
    }
    
    fn get_ddr_register_bit_field(pin: &Pin) -> u8 {
        match pin {
            Pin::Zero       => DDRDBitField::PD0 as u8,
            Pin::One        => DDRDBitField::PD1 as u8,
            Pin::Two        => DDRDBitField::PD2 as u8,
            Pin::Three      => DDRDBitField::PD3 as u8,
            Pin::Four       => DDRDBitField::PD4 as u8,
            Pin::Five       => DDRDBitField::PD5 as u8,
            Pin::Six        => DDRDBitField::PD6 as u8,
            Pin::Seven      => DDRDBitField::PD7 as u8,
            Pin::Eight      => DDRBBitField::PB0 as u8,
            Pin::Nine       => DDRBBitField::PB1 as u8,
            Pin::Ten        => DDRBBitField::PB2 as u8,
            Pin::Eleven     => DDRBBitField::PB3 as u8,
            Pin::Twelve     => DDRBBitField::PB4 as u8,
            Pin::Thirteen   => DDRBBitField::PB5 as u8,
            Pin::Fourteen   => DDRCBitField::AC0 as u8,
            Pin::Fifteen    => DDRCBitField::AC1 as u8,
            Pin::Sixteen    => DDRCBitField::AC2 as u8,
            Pin::Seventeen  => DDRCBitField::AC3 as u8,
            Pin::Eighteen   => DDRCBitField::AC4 as u8,
            Pin::Nineteen   => DDRCBitField::AC5 as u8,
            // Pin::Twenty     => AC6 as u8,
            // Pin::TwentyOne  => AC7 as u8,
        }
    }
}
