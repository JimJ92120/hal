#[cfg(feature = "arduino-uno")]
pub mod arduino_uno;

#[cfg(any(
    feature = "rpi-1a",
    feature = "rpi-1ap",
    feature = "rpi-1b",
    feature = "rpi-1bp",
    feature = "rpi-cm1",
    feature = "rpi-0",
    feature = "rpi-0w",
    feature = "rpi-2b1",
    feature = "rpi-2b2",
    feature = "rpi-2b3",
    feature = "rpi-3b",
    feature = "rpi-cm3",
    feature = "rpi-02",
    feature = "rpi-02w",
    feature = "rpi-3ap",
    feature = "rpi-3bp",
    feature = "rpi-cm3p",
    feature = "rpi-4",
    feature = "rpi-400",
    feature = "rpi-cm4",
    feature = "rpi-cm4s",
))]
pub mod rpi;
