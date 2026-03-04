use lib_peripherals::arduino_uno::{ Pin, LED };

use crate::helpers;

pub fn run() {
    const DELAY_DURATION: u32 = 1_000_000;
    
    let mut led = LED::new(Pin::Thirteen);

    loop {
        led.toggle();
        helpers::delay(DELAY_DURATION);
    }
}
