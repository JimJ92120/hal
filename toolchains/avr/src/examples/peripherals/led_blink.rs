use lib_peripherals::arduino_uno::{ Pin, LED };

use crate::helpers;

pub fn run() {
    const DELAY_DURATION: u32 = 1_000_000;
    // GPIO 13 / LED_BUILTIN
    const PIN: Pin = Pin::Thirteen;
    
    LED::init(PIN);

    loop {
        LED::toggle(PIN);
        helpers::delay(DELAY_DURATION);
    }
}
