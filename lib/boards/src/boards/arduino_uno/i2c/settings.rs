#[derive(Debug)]
pub enum I2CBitRate {
    One,
    Four,
    Sixteen,
    SixtyFour,
}

#[derive(Debug)]
pub struct I2CSettings {
    pub bit_rate: I2CBitRate,
    pub baud_rate: u8,
    pub slave_address: u8,
}
