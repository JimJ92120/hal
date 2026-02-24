pub const MMIO_BASE_OFFSET: u32 = 0x3F000000;

// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=8

mod register_2097152_0x00200000_gpfsel0;
mod register_2097156_0x00200004_gpfsel1;
mod register_2097160_0x00200008_gpfsel2;
mod register_2097164_0x0020000c_gpfsel3;
mod register_2097168_0x00200010_gpfsel4;
mod register_2097172_0x00200014_gpfsel5;
mod register_2097180_0x0020001c_gpset0;
mod register_2097184_0x00200020_gpset1;
mod register_2097192_0x00200028_gpclr0;
mod register_2097196_0x0020002c_gpclr1;

pub use register_2097152_0x00200000_gpfsel0::GPFSEL0;
pub use register_2097156_0x00200004_gpfsel1::GPFSEL1;
pub use register_2097160_0x00200008_gpfsel2::GPFSEL2;
pub use register_2097164_0x0020000c_gpfsel3::GPFSEL3;
pub use register_2097168_0x00200010_gpfsel4::GPFSEL4;
pub use register_2097172_0x00200014_gpfsel5::GPFSEL5;
pub use register_2097180_0x0020001c_gpset0::GPSET0;
pub use register_2097184_0x00200020_gpset1::GPSET1;
pub use register_2097192_0x00200028_gpclr0::GPCLR0;
pub use register_2097196_0x0020002c_gpclr1::GPCLR1;
