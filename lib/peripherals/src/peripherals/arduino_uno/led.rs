use lib_boards::arduino_uno::{ GPIO, Digital };
use super::Pin;

pub struct LED;

impl LED {
    pub fn init(pin: Pin) {
        GPIO::set_output(pin);
    }

    pub fn on(pin: Pin) {
        Digital::set_high(pin);
    }

    pub fn off(pin: Pin) {
        Digital::set_low(pin);
    }

    pub fn toggle(pin: Pin) {
        if Self::is_on(pin) {
            Self::off(pin);
        } else {
            Self::on(pin);
        }
    }

    pub fn is_on(pin: Pin) -> bool {
        Digital::read_state(pin)
    }
}
