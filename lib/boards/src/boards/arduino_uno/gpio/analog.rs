use lib_registers::{
    globals::Register,
    atmega328p::{
        ADMUX, ADMUXBitField,
        ADCSRA, ADCSRABitField,
        ADCL, ADCH,
    }
};

use super::Pin;

// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=217
#[derive(Debug)]
pub enum AnalogMode {
    NoVref,
    AVcc,
    Internal1_1,
}

// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=219
#[derive(Debug)]
pub enum AnalogPrescaler {
    Default,
    Two,
    Four,
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
    HundredTwentyEight,
}

#[derive(Debug)]
pub struct AnalogSettings {
    pub mode: AnalogMode,
    pub prescaler: AnalogPrescaler,
}

#[derive(Debug)]
pub struct Analog;

impl Analog {
    pub fn init() {
        ADCSRA::or(1 << ADCSRABitField::ADEN as u8);
    }

    pub fn start_conversion(pin: Pin, settings: AnalogSettings) {
        Self::set_pin(pin);
        Self::set_mode(settings.mode);
        Self::set_prescaler(settings.prescaler);

        ADCSRA::or(1 << ADCSRABitField::ADSC as u8);
    }

    pub fn read() -> u16 {
        while 0 != ADCSRA::get() & (1 << ADCSRABitField::ADSC as u8) {}

        (ADCL::get() as u16) | ((ADCH::get() as u16) << 8)
    }

    fn set_mode(mode: AnalogMode) {
        match mode {
            AnalogMode::NoVref => (),
            AnalogMode::AVcc => ADMUX::or(1 << ADMUXBitField::REFS0 as u8),
            AnalogMode::Internal1_1 => {
                ADMUX::or(1 << ADMUXBitField::REFS1 as u8);
                ADMUX::or(1 << ADMUXBitField::REFS0 as u8);
            }
        };
    }

    fn set_prescaler(prescaler: AnalogPrescaler) {
        match prescaler {
            AnalogPrescaler::Default => (),
            AnalogPrescaler::Two => ADCSRA::or(1 << ADCSRABitField::ADPS0 as u8),
            AnalogPrescaler::Four => ADCSRA::or(1 << ADCSRABitField::ADPS1 as u8),
            AnalogPrescaler::Eight => {
                ADCSRA::or(1 << ADCSRABitField::ADPS0 as u8);
                ADCSRA::or(1 << ADCSRABitField::ADPS1 as u8);
            },
            AnalogPrescaler::Sixteen => ADCSRA::or(1 << ADCSRABitField::ADPS2 as u8),
            AnalogPrescaler::ThirtyTwo => {
                ADCSRA::or(1 << ADCSRABitField::ADPS0 as u8);
                ADCSRA::or(1 << ADCSRABitField::ADPS2 as u8);
            },
            AnalogPrescaler::SixtyFour => {
                ADCSRA::or(1 << ADCSRABitField::ADPS1 as u8);
                ADCSRA::or(1 << ADCSRABitField::ADPS2 as u8);
            },
            AnalogPrescaler::HundredTwentyEight => {
                ADCSRA::or(1 << ADCSRABitField::ADPS0 as u8);
                ADCSRA::or(1 << ADCSRABitField::ADPS1 as u8);
                ADCSRA::or(1 << ADCSRABitField::ADPS2 as u8);
            },
        };
    }

    fn set_pin(pin: Pin) {
        match pin {
            Pin::Fourteen => (),
            Pin::Fifteen => ADMUX::or(1 << ADMUXBitField::MUX0 as u8),
            Pin::Sixteen => ADMUX::or(1 << ADMUXBitField::MUX1 as u8),
            Pin::Seventeen => {
                ADMUX::or(1 << ADMUXBitField::MUX0 as u8);
                ADMUX::or(1 << ADMUXBitField::MUX1 as u8);
            },
            Pin::Eighteen => ADMUX::or(1 << ADMUXBitField::MUX2 as u8),
            Pin::Nineteen => {
                ADMUX::or(1 << ADMUXBitField::MUX0 as u8);
                ADMUX::or(1 << ADMUXBitField::MUX2 as u8);
            },
            // Pin::Twenty => {
            //     ADMUX::or(1 << ADMUXBitField::MUX1 as u8);
            //     ADMUX::or(1 << ADMUXBitField::MUX2 as u8);
            // },
            // Pin::TwentyOne => {
            //     ADMUX::or(1 << ADMUXBitField::MUX0 as u8);
            //     ADMUX::or(1 << ADMUXBitField::MUX1 as u8);
            //     ADMUX::or(1 << ADMUXBitField::MUX2 as u8);
            // },

            _ => panic!("Pin doesn't support analog."),
        };
    }
}
