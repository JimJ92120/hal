use lib_boards::arduino_uno::{ GPIO, Digital };
use super::Pin;

#[derive(Debug, Clone)]
pub struct LEDState {
    pub is_on: bool,
}

#[derive(Debug)]
pub struct LED {
    pin: Pin,
    state: LEDState,
}

impl LED {
    pub fn new(pin: Pin) -> Self {
        GPIO::set_output(pin);

        Self {
            pin,
            state: LEDState {
                is_on: false,
            }
        }
    }

    pub fn state(&self) -> LEDState {
        self.state.clone()
    }

    pub fn on(&mut self) {
        Digital::set_high(self.pin);

        self.state.is_on = true;
    }

    pub fn off(&mut self) {
        Digital::set_low(self.pin);

        self.state.is_on = false;
    }

    pub fn toggle(&mut self) {
        if self.state.is_on {
            self.off();
        } else {
            self.on();
        }
    }
}
