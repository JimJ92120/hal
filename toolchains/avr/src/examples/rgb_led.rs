use lib_boards::arduino_uno::{ Pin, GPIO };

use crate::helpers;

pub fn run() {
    const DELAY_DURATION: u32 = 50_000;
    // GPIO 5 (red)
    const LED_RED: Pin = Pin::Three;
    // GPIO 6 (green)
    const LED_GREEN: Pin = Pin::Six;
    // GPIO 9 (blue)
    const LED_BLUE: Pin = Pin::Nine;
    
    GPIO::set_output(LED_RED);
    GPIO::set_output(LED_GREEN);
    GPIO::set_output(LED_BLUE);

    GPIO::init_pwm_timer(LED_RED);
    GPIO::init_pwm_timer(LED_GREEN);
    GPIO::init_pwm_timer(LED_BLUE);

    let mut cycle: u8 = 0;
    let mut increment: bool = true;

    loop {
        GPIO::set_pwm_cycle(LED_RED, cycle);
        GPIO::set_pwm_cycle(LED_GREEN, 255 - cycle);
        GPIO::set_pwm_cycle(LED_BLUE, cycle);

        if increment {
            cycle += 5;
        } else {
            cycle -= 5;
        }

        if 0 == cycle && !increment {
            increment = true;
        } else if 255 == cycle && increment {
            increment = false;
        }

        helpers::delay(DELAY_DURATION);
    }
}
