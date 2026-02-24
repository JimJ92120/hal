#![no_std]
#![no_main]

mod panic;
mod boot;
mod helpers;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    const DELAY_DURATION: u32 = 3_000_000;

    loop {

        helpers::delay(DELAY_DURATION);
    }
}
