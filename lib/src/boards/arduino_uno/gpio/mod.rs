use crate::{
    globals::{ Bit, Register },
    registers::atmega328p::{
        DDRB, PORTB,
        DDRC, PORTC,
        DDRD, PORTD,
    }
};

mod pin;

pub use pin::Pin;

#[derive(Debug)]
pub struct GPIO;

impl GPIO {
    pub fn set_output(pin: Pin) {
        let bit_mask = pin.bit().mask();

        match pin {
            Pin::Zero | Pin::One | Pin::Two | Pin::Three | Pin::Four | Pin::Five | Pin::Six | Pin::Seven
                => unsafe {
                    DDRD::set(bit_mask);
                },
            Pin::Eight | Pin::Nine | Pin::Ten | Pin::Eleven | Pin::Twelve | Pin::Thirteen
                => unsafe {
                    DDRB::set(bit_mask);
                },
            Pin::Fourteen | Pin::Fifteen | Pin::Sixteen | Pin::Seventeen | Pin::Eighteen | Pin::Nineteen
                => unsafe {
                    DDRC::set(bit_mask);
                },
        };
    }

    pub fn set_input(pin: Pin) {
        let bit_mask = pin.bit().mask();

        match pin {
            Pin::Zero | Pin::One | Pin::Two | Pin::Three | Pin::Four | Pin::Five | Pin::Six | Pin::Seven
                => unsafe {
                    DDRD::set(DDRD::get() & !bit_mask);
                },
            Pin::Eight | Pin::Nine | Pin::Ten | Pin::Eleven | Pin::Twelve | Pin::Thirteen
                => unsafe {
                    DDRD::set(DDRD::get() & !bit_mask);
                },
            Pin::Fourteen | Pin::Fifteen | Pin::Sixteen | Pin::Seventeen | Pin::Eighteen | Pin::Nineteen
                => unsafe {
                    DDRD::set(DDRD::get() & !bit_mask);
                },
        };
    }

    pub fn set_high(pin: Pin) {
        let bit_mask = pin.bit().mask();

        match pin {
            Pin::Zero | Pin::One | Pin::Two | Pin::Three | Pin::Four | Pin::Five | Pin::Six | Pin::Seven
                => unsafe {
                    PORTD::set(bit_mask);
                },
            Pin::Eight | Pin::Nine | Pin::Ten | Pin::Eleven | Pin::Twelve | Pin::Thirteen
                => unsafe {
                    PORTB::set(bit_mask);
                },
            Pin::Fourteen | Pin::Fifteen | Pin::Sixteen | Pin::Seventeen | Pin::Eighteen | Pin::Nineteen
                => unsafe {
                    PORTC::set(bit_mask);
                },
        };
    }

    pub fn set_low(pin: Pin) {
        let bit_mask = pin.bit().mask();

        match pin {
            Pin::Zero | Pin::One | Pin::Two | Pin::Three | Pin::Four | Pin::Five | Pin::Six | Pin::Seven
                => unsafe {
                    PORTD::set(PORTD::get() & !bit_mask);
                },
            Pin::Eight | Pin::Nine | Pin::Ten | Pin::Eleven | Pin::Twelve | Pin::Thirteen
                => unsafe {
                    PORTB::set(PORTB::get() & !bit_mask);
                },
            Pin::Fourteen | Pin::Fifteen | Pin::Sixteen | Pin::Seventeen | Pin::Eighteen | Pin::Nineteen
                => unsafe {
                    PORTC::set(PORTC::get() & !bit_mask);
                },
        };
    }
}
