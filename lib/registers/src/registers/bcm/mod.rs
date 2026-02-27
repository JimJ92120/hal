pub const MMIO_BASE: u32 = if cfg!(feature = "bcm2835") {
    0x20204000
} else if cfg!(feature = "bcm2836")
    || cfg!(feature = "bcm2837")
    || cfg!(feature = "rp3a0")
    || cfg!(feature = "bcm2837b0")
{
    0x3F000000
} else if cfg!(feature = "bcm2711") {
    0x7E000000
} else {
    panic!("No MMIO_BASE defined");
};

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

pub use register_2097152_0x00200000_gpfsel0::{ GPFSEL0, GPFSEL0BitField };
pub use register_2097156_0x00200004_gpfsel1::{ GPFSEL1, GPFSEL1BitField };
pub use register_2097160_0x00200008_gpfsel2::{ GPFSEL2, GPFSEL2BitField };
pub use register_2097164_0x0020000c_gpfsel3::{ GPFSEL3, GPFSEL3BitField };
pub use register_2097168_0x00200010_gpfsel4::{ GPFSEL4, GPFSEL4BitField };
pub use register_2097172_0x00200014_gpfsel5::{ GPFSEL5, GPFSEL5BitField };
pub use register_2097180_0x0020001c_gpset0::{ GPSET0, GPSET0BitField };
pub use register_2097184_0x00200020_gpset1::{ GPSET1, GPSET1BitField };
pub use register_2097192_0x00200028_gpclr0::{ GPCLR0, GPCLR0BitField };
pub use register_2097196_0x0020002c_gpclr1::{ GPCLR1, GPCLR1BitField };
