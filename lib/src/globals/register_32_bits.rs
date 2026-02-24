pub type Address = *mut u32;

pub trait Register {
    const ADDRESS: Address;
    
    unsafe fn get() -> u32 {
        unsafe {
            core::ptr::read_volatile(Self::ADDRESS)
        }
    }

    unsafe fn set(value: u32) {
        unsafe {
            core::ptr::write_volatile(Self::ADDRESS, value);
        }
    }
}

#[derive(Debug)]
#[repr(u32)]
pub enum Bit {
    Zero        = 0,
    One         = 1,
    Two         = 2,
    Three       = 3,
    Four        = 4,
    Five        = 5,
    Six         = 6,
    Seven       = 7,
    Eight       = 8,
    Nine        = 9,
    Ten         = 10,
    Eleven      = 11,
    Twelve      = 12,
    Thirteen    = 13,
    Fourteen    = 14,
    Fifteen     = 15,
    Sixteen     = 16,
    Seventeen   = 17,
    Eighteen    = 18,
    Nineteen    = 19,
    Twenty      = 20,
    TwentyOne   = 21,
    TwentyTwo   = 22,
    TwentyThree = 23,
    TwentyFour  = 24,
    TwentyFive  = 25,
    TwentySix   = 26,
    TwentySeven = 27,
    TwentyEight = 28,
    TwentyNine  = 29,
    Thirty      = 30,
    ThirtyOne   = 31,
}
impl Bit {
    // 1 << value
    pub fn mask(&self) -> u32 {
        match self {
            Bit::Zero        => 0b0000_0000_0000_0000_0000_0000_0000_0001,
            Bit::One         => 0b0000_0000_0000_0000_0000_0000_0000_0010,
            Bit::Two         => 0b0000_0000_0000_0000_0000_0000_0000_0100,
            Bit::Three       => 0b0000_0000_0000_0000_0000_0000_0000_1000,
            Bit::Four        => 0b0000_0000_0000_0000_0000_0000_0001_0000,
            Bit::Five        => 0b0000_0000_0000_0000_0000_0000_0010_0000,
            Bit::Six         => 0b0000_0000_0000_0000_0000_0000_0100_0000,
            Bit::Seven       => 0b0000_0000_0000_0000_0000_0000_1000_0000,
            Bit::Eight       => 0b0000_0000_0000_0000_0000_0001_0000_0000,
            Bit::Nine        => 0b0000_0000_0000_0000_0000_0010_0000_0000,
            Bit::Ten         => 0b0000_0000_0000_0000_0000_0100_0000_0000,
            Bit::Eleven      => 0b0000_0000_0000_0000_0000_1000_0000_0000,
            Bit::Twelve      => 0b0000_0000_0000_0000_0001_0000_0000_0000,
            Bit::Thirteen    => 0b0000_0000_0000_0000_0010_0000_0000_0000,
            Bit::Fourteen    => 0b0000_0000_0000_0000_0100_0000_0000_0000,
            Bit::Fifteen     => 0b0000_0000_0000_0000_1000_0000_0000_0000,
            Bit::Sixteen     => 0b0000_0000_0000_0001_0000_0000_0000_0000,
            Bit::Seventeen   => 0b0000_0000_0000_0010_0000_0000_0000_0000,
            Bit::Eighteen    => 0b0000_0000_0000_0100_0000_0000_0000_0000,
            Bit::Nineteen    => 0b0000_0000_0000_1000_0000_0000_0000_0000,
            Bit::Twenty      => 0b0000_0000_0001_0000_0000_0000_0000_0000,
            Bit::TwentyOne   => 0b0000_0000_0010_0000_0000_0000_0000_0000,
            Bit::TwentyTwo   => 0b0000_0000_0100_0000_0000_0000_0000_0000,
            Bit::TwentyThree => 0b0000_0000_1000_0000_0000_0000_0000_0000,
            Bit::TwentyFour  => 0b0000_0001_0000_0000_0000_0000_0000_0000,
            Bit::TwentyFive  => 0b0000_0010_0000_0000_0000_0000_0000_0000,
            Bit::TwentySix   => 0b0000_0100_0000_0000_0000_0000_0000_0000,
            Bit::TwentySeven => 0b0000_1000_0000_0000_0000_0000_0000_0000,
            Bit::TwentyEight => 0b0001_0000_0000_0000_0000_0000_0000_0000,
            Bit::TwentyNine  => 0b0010_0000_0000_0000_0000_0000_0000_0000,
            Bit::Thirty      => 0b0100_0000_0000_0000_0000_0000_0000_0000,
            Bit::ThirtyOne   => 0b1000_0000_0000_0000_0000_0000_0000_0000,
        }
    }
}
