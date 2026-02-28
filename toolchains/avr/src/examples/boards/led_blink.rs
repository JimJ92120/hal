use lib_boards::arduino_uno::{ Pin, GPIO, Digital };

use crate::helpers;

pub fn run() {
    const DELAY_DURATION: u32 = 1_000_000;
    // GPIO 13 / LED_BUILTIN
    const PIN: Pin = Pin::Thirteen;
    
    GPIO::set_output(PIN);

    loop {
        Digital::set_high(PIN);
        helpers::delay(DELAY_DURATION);

        Digital::set_low(PIN);
        helpers::delay(DELAY_DURATION);
    }
}
