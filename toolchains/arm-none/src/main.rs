#![no_std]
#![no_main]

use hal_lib::{
    boards::rpi::{ Pin, GPIO }
};

mod panic;
mod boot;
mod helpers;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    const DELAY_DURATION: u32 = 3_000_000;
    // GPIO 27
    const PIN: Pin = Pin::TwentySeven;

    GPIO::set_output(PIN);

    loop {
        GPIO::set_high(PIN);
        helpers::delay(DELAY_DURATION);

        GPIO::set_low(PIN);
        helpers::delay(DELAY_DURATION);
    }
}
