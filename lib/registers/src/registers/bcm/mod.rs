pub const MMIO_BASE: u32 = if cfg!(feature = "bcm2835") {
    0x20204000
} else if cfg!(any(
    feature = "bcm2836",
    feature = "bcm2837",
    feature = "rp3a0",
    feature = "bcm2837b0",
)) {
    0x3F000000
} else if cfg!(feature = "bcm2711") {
    0x7E000000
} else {
    // panic!("No MMIO_BASE defined");
    0x3F000000
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
mod register_2183168_0x00215000_aux_irq;
mod register_2183172_0x00215004_aux_enables;
mod register_2183232_0x00215040_aux_mu_io_reg;
mod register_2183236_0x00215044_aux_mu_iir_reg;
mod register_2183240_0x00215048_aux_mu_ier_reg;
mod register_2183244_0x0021504c_aux_mu_lcr_reg;
mod register_2183248_0x00215050_aux_mu_mcr_reg;
mod register_2183252_0x00215054_aux_mu_lsr_reg;
mod register_2183256_0x00215058_aux_mu_msr_reg;
mod register_2183260_0x0021505c_aux_mu_scratch;
mod register_2183264_0x00215060_aux_mu_cntl_reg;
mod register_2183268_0x00215064_aux_mu_stat_reg;
mod register_2183272_0x00215068_aux_mu_baud_reg;

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
pub use register_2183168_0x00215000_aux_irq::{ AUX_IRQ, AUX_IRQ_BitField };
pub use register_2183172_0x00215004_aux_enables::{ AUX_ENABLES, AUX_ENABLES_BitField };
pub use register_2183232_0x00215040_aux_mu_io_reg::{ AUX_MU_IO_REG, AUX_MU_IO_REG_BitField };
pub use register_2183236_0x00215044_aux_mu_iir_reg::{ AUX_MU_IIR_REG, AUX_MU_IIR_REG_BitField };
pub use register_2183240_0x00215048_aux_mu_ier_reg::{ AUX_MU_IER_REG, AUX_MU_IER_REG_BitField };
pub use register_2183244_0x0021504c_aux_mu_lcr_reg::{ AUX_MU_LCR_REG, AUX_MU_LCR_REG_BitField };
pub use register_2183248_0x00215050_aux_mu_mcr_reg::{ AUX_MU_MCR_REG, AUX_MU_MCR_REG_BitField };
pub use register_2183252_0x00215054_aux_mu_lsr_reg::{ AUX_MU_LSR_REG, AUX_MU_LSR_REG_BitField };
pub use register_2183256_0x00215058_aux_mu_msr_reg::{ AUX_MU_MSR_REG, AUX_MU_MSR_REG_BitField };
pub use register_2183260_0x0021505c_aux_mu_scratch::{ AUX_MU_SCRATCH, AUX_MU_SCRATCH_BitField };
pub use register_2183264_0x00215060_aux_mu_cntl_reg::{ AUX_MU_CNTL_REG, AUX_MU_CNTL_REG_BitField };
pub use register_2183268_0x00215064_aux_mu_stat_reg::{ AUX_MU_STAT_REG, AUX_MU_STAT_REG_BitField };
pub use register_2183272_0x00215068_aux_mu_baud_reg::{ AUX_MU_BAUD_REG, AUX_MU_BAUD_REG_BitField };
