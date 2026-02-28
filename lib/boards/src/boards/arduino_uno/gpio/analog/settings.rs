// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=217
#[derive(Debug)]
pub enum AnalogMode {
    NoVref,
    AVcc,
    Internal1_1,
}

// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=219
#[derive(Debug)]
pub enum AnalogPrescaler {
    Default,
    Two,
    Four,
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
    HundredTwentyEight,
}

#[derive(Debug)]
pub struct AnalogSettings {
    pub mode: AnalogMode,
    pub prescaler: AnalogPrescaler,
}
