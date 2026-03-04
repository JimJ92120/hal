use lib_peripherals::arduino_uno::{ Pin, LED, Button };

use crate::helpers;

pub fn run() {    
    let mut led = LED::new(Pin::Thirteen);
    let button = Button::new(Pin::Seven);

    loop {
        if button.is_pressed() {
            led.on();
        } else {
            led.off();
        }
    }
}

