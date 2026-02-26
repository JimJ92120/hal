#[cfg(feature = "atmega328p")]
pub mod atmega328p;

// see bcm/README.md
#[cfg(any(
    feature = "bcm2835",
    feature = "bcm2836",
    feature = "bcm2837",
    feature = "bcm2837b0",
    feature = "bcm2711",
))]
pub mod bcm;
