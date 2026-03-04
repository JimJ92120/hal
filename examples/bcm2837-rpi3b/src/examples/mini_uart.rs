use lib_boards::rpi::{ MiniUART };

use crate::helpers;

pub fn run() {
    const DELAY_DURATION: u32 = 120_000_000;
    const BAUD_RATE: u32 = 57_600;
    const FREQUENCY: u32 = 1_200_000_000;

    MiniUART::init(BAUD_RATE, FREQUENCY);

    MiniUART::send("hello world\n");

    loop {
        MiniUART::send("hello world\n");
        helpers::delay(DELAY_DURATION);
    }
}
