#[derive(Debug)]
pub enum UARTCharSize {
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug)]
pub enum UARTStopBit {
    One,
    Two,
}

#[derive(Debug)]
pub enum UARTSyncMode {
    Async,
    Sync,
    SPI,
}

#[derive(Debug)]
pub enum UARTParityMode {
    Disabled,
    Even,
    Odd,
}

#[derive(Debug)]
pub struct UARTSettings {
    pub baud_rate: u32,
    pub frequency: u32,
    pub enable_reception: bool,
    pub enable_transmission: bool,
    pub char_size: UARTCharSize,
    pub stop_bit: UARTStopBit,
    pub sync_mode: UARTSyncMode,
    pub parity_mode: UARTParityMode,
}
