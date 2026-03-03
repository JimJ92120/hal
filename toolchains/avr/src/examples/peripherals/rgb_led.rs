use lib_peripherals::arduino_uno::{ Pin, RGB };

use crate::helpers;

pub fn run() {
    const DELAY_DURATION: u32 = 1_000_000;
    
    let mut led = RGB::new(
        Pin::Eleven,
        Pin::Five,
        Pin::Three
    );

    loop {
        led.rgb(255, 0, 0); 
        helpers::delay(DELAY_DURATION);

        led.rgb(0, 255, 0); 
        helpers::delay(DELAY_DURATION);

        led.rgb(0, 0, 255);
        helpers::delay(DELAY_DURATION);
    }
}
