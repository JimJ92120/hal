mod register_8_bits;

#[cfg(feature = "atmega328p")]
pub use register_8_bits::{ Address, Register, Bit };
