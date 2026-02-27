use lib_boards::arduino_uno::{ Pin, GPIO, Digital };

pub fn run() {
    // GPIO 13 / LED_BUILTIN
    const PIN: Pin = Pin::Thirteen;
    // GPIO 7
    const BUTTON: Pin = Pin::Seven;
    
    GPIO::set_output(PIN);
    GPIO::set_input(BUTTON);

    loop {
        if Digital::read_state(BUTTON) {
            Digital::set_high(PIN);
        } else {
            Digital::set_low(PIN);
        }
    }
}
