use lib_boards::rpi::{ Pin, GPIO };

use crate::helpers;

pub fn run() {
    const DELAY_DURATION: u32 = 3_000_000;
    const PIN: Pin = Pin::TwentySeven;

    GPIO::set_output(PIN);

    loop {
        GPIO::set_high(PIN);
        helpers::delay(DELAY_DURATION);

        GPIO::set_low(PIN);
        helpers::delay(DELAY_DURATION);
    }
}
