// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=217
#[derive(Debug, Clone)]
pub enum AnalogMode {
    NoVref,
    AVcc,
    Internal1_1,
}

// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=219
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct AnalogSettings {
    pub mode: AnalogMode,
    pub prescaler: AnalogPrescaler,
}
