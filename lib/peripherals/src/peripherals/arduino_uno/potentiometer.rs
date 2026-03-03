use lib_boards::arduino_uno::{ GPIO, Analog };

use super::{ 
    Pin,
    AnalogSettings, AnalogMode, AnalogPrescaler,
};

#[derive(Debug)]
pub struct Potentiometer {
    pin: Pin,
    analog_settings:  AnalogSettings,
}

impl Potentiometer {
    pub fn new(pin: Pin, analog_settings: AnalogSettings) -> Self {
        GPIO::set_input(pin);
        Analog::init();

        Self {
            pin,
            analog_settings,
        }
    }

    pub fn value(&self) -> u16 {
        Analog::start_conversion(self.pin, self.analog_settings.clone());
    
        Analog::read()
    }
}
