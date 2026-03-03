use super::{
    Pin,
    super::{ Timer, Timer0, Timer1, Timer2 }
};

#[derive(Debug)]
pub struct PWM;

impl PWM {
    pub fn init_pwm_timer(pin: Pin) {
        match pin {
            Pin::Three | Pin::Eleven => Timer2::init(),
            Pin::Five | Pin::Six => Timer0::init(),
            Pin::Nine | Pin::Ten => Timer1::init(),
            _ => panic!("Pin doesn't support PWM."),
        };
    }

    pub fn set_pwm_cycle(pin: Pin, value: u8) {
        match pin {
            Pin::Three => Timer2::set_b(value),
            Pin::Five => Timer0::set_b(value),
            Pin::Six => Timer0::set_a(value),
            Pin::Nine => Timer1::set_a(value as u16),
            Pin::Ten => Timer1::set_b(value as u16),
            Pin::Eleven => Timer2::set_a(value),

            _ => panic!("Pin doesn't support PWM."),
        };
    }
}
