pub type Address = *mut u8;

pub trait Register {
    const ADDRESS: Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum Bit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
}
impl Bit {
    // 1 << value
    pub fn mask(&self) -> u8 {
        match self {
            Bit::Zero   => 0b0000_0001,
            Bit::One    => 0b0000_0010,
            Bit::Two    => 0b0000_0100,
            Bit::Three  => 0b0000_1000,
            Bit::Four   => 0b0001_0000,
            Bit::Five   => 0b0010_0000,
            Bit::Six    => 0b0100_0000,
            Bit::Seven  => 0b1000_0000,
        }
    }
}
