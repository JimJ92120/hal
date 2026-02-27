use lib_boards::arduino_uno::{ Pin, GPIO };

pub fn run() {
    // GPIO 13 / LED_BUILTIN
    const PIN: Pin = Pin::Thirteen;
    // GPIO 7
    const BUTTON: Pin = Pin::Seven;
    
    GPIO::set_output(PIN);
    GPIO::set_input(BUTTON);

    loop {
        if GPIO::read_state(BUTTON) {
            GPIO::set_high(PIN);
        } else {
            GPIO::set_low(PIN);
        }
    }
}
