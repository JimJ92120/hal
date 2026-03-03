use lib_boards::arduino_uno::{ GPIO, Digital };
use super::Pin;

#[derive(Debug)]
pub struct Button {
    pin: Pin,
}

impl Button {
    pub fn new(pin: Pin) -> Self {
        GPIO::set_input(pin);

        Self {
            pin,
        }
    }

    pub fn is_pressed(&self) -> bool {
        Digital::read_state(self.pin)
    }
}
