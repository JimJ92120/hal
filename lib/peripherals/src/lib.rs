#![no_std]
#![no_main]

#![allow(dead_code)]
#![allow(unused_imports)]

mod peripherals;

#[cfg(feature = "arduino-uno")]
pub use peripherals::arduino_uno;
