mod gpio;
mod uart;
mod timer;
mod i2c;

pub use gpio::{
    Pin, GPIO, Digital, PWM,
    Analog, AnalogSettings, AnalogMode, AnalogPrescaler
};
pub use uart::{
    UART, UARTSettings, UARTCharSize, UARTStopBit, UARTSyncMode, UARTParityMode,
};
pub use timer::{ Timer, Timer0, Timer1, Timer2 };
pub use i2c::{ I2C, I2CSettings, I2CBitRate };
