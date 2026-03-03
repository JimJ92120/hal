use lib_boards::arduino_uno::{ GPIO, Digital, PWM };
use super::Pin;

#[derive(Debug, Clone)]
pub struct RGBState {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug)]
pub struct RGB {
    red_pin: Pin,
    green_pin: Pin,
    blue_pin: Pin,
    state: RGBState,
}

impl RGB {
    pub fn new(
        red_pin: Pin,
        green_pin: Pin,
        blue_pin: Pin,
    ) -> Self {
        for pin in [red_pin, green_pin, blue_pin] {
            GPIO::set_output(pin);
            PWM::init_pwm_timer(pin);
        }

        Self {
            red_pin,
            green_pin,
            blue_pin,
            state: RGBState {
                red: 0,
                green: 0,
                blue: 0,
            }
        }
    }

    pub fn state(&self) -> RGBState {
        self.state.clone()
    }

    pub fn red(&mut self, value: u8) {
        PWM::set_pwm_cycle(self.red_pin, value);

        self.state.red = value;
    }

    pub fn green(&mut self, value: u8) {
        PWM::set_pwm_cycle(self.green_pin, value);

        self.state.green = value;
    }

    pub fn blue(&mut self, value: u8) {
        PWM::set_pwm_cycle(self.blue_pin, value);

        self.state.blue = value;
    }

    pub fn rgb(&mut self, red: u8, green: u8, blue: u8) {
        self.red(red);
        self.green(green);
        self.blue(blue);
    }
}
