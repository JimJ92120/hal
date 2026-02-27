// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=19
// When addressing I/O registers as data space using LD and ST instructions, 0x20 must be added to these addresses
pub const IO_OFFSET: u8 = 0x20;

// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=280
mod register_003_0x03_pinb;
mod register_004_0x04_ddrb;
mod register_005_0x05_portb;
mod register_006_0x06_pinc;
mod register_007_0x07_ddrc;
mod register_008_0x08_portc;
mod register_009_0x09_pind;
mod register_010_0x0a_ddrd;
mod register_011_0x0b_portd;
mod register_021_0x15_tifr0;
mod register_022_0x16_tifr1;
mod register_023_0x17_tifr2;
mod register_027_0x1b_pcifr;
mod register_028_0x1c_eifr;
mod register_029_0x1d_eimsk;
mod register_030_0x1e_gpior0;
mod register_031_0x1f_eecr;
mod register_032_0x20_eedr;
mod register_033_0x21_eearl;
mod register_034_0x22_eearh;

pub use register_003_0x03_pinb::{ PINB, PINBBitField };
pub use register_004_0x04_ddrb::{ DDRB, DDRBBitField };
pub use register_005_0x05_portb::{ PORTB, PORTBBitField };
pub use register_006_0x06_pinc::{ PINC, PINCBitField };
pub use register_007_0x07_ddrc::{ DDRC, DDRCBitField };
pub use register_008_0x08_portc::{ PORTC, PORTCBitField };
pub use register_009_0x09_pind::{ PIND, PINDBitField };
pub use register_010_0x0a_ddrd::{ DDRD, DDRDBitField };
pub use register_011_0x0b_portd::{ PORTD, PORTDBitField };
pub use register_021_0x15_tifr0::{ TIFR0, TIFR0BitField };
pub use register_022_0x16_tifr1::{ TIFR1, TIFR1BitField };
pub use register_023_0x17_tifr2::{ TIFR2, TIFR2BitField };
pub use register_027_0x1b_pcifr::{ PCIFR, PCIFRBitField };
pub use register_028_0x1c_eifr::{ EIFR, EIFRBitField };
pub use register_029_0x1d_eimsk::{ EIMSK, EIMSKBitField };
pub use register_030_0x1e_gpior0::GPIOR0;
pub use register_031_0x1f_eecr::{ EECR, EECRBitField };
pub use register_032_0x20_eedr::EEDR;
pub use register_033_0x21_eearl::EEARL;
pub use register_034_0x22_eearh::EEARH;

// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=279
mod register_035_0x23_gtccr;
mod register_036_0x24_tccr0a;
mod register_037_0x25_tccr0b;
mod register_038_0x26_tcnt0;
mod register_039_0x27_ocr0a;
mod register_040_0x28_ocr0b;
mod register_042_0x2a_gpior1;
mod register_043_0x2b_gpior2;
mod register_044_0x2c_spcr;
mod register_045_0x2d_spsr;
mod register_046_0x2e_spdr;
mod register_048_0x30_acsr;
mod register_051_0x33_smcr;
mod register_052_0x34_mcusr;
mod register_053_0x35_mcucr;
mod register_055_0x37_spmcsr;
mod register_061_0x3d_spl;
mod register_062_0x3e_sph;
mod register_063_0x3f_sreg;
mod register_096_0x60_wdtcsr;
mod register_097_0x61_clkpr;
mod register_100_0x64_prr;
mod register_102_0x66_osccal;
mod register_104_0x68_pcicr;

pub use register_035_0x23_gtccr::{ GTCCR, GTCCRBitField };
pub use register_036_0x24_tccr0a::{ TCCR0A, TCCR0ABitField };
pub use register_037_0x25_tccr0b::{ TCCR0B, TCCR0BBitField };
pub use register_038_0x26_tcnt0::TCNT0;
pub use register_039_0x27_ocr0a::OCR0A;
pub use register_040_0x28_ocr0b::OCR0B;
pub use register_042_0x2a_gpior1::GPIOR1;
pub use register_043_0x2b_gpior2::GPIOR2;
pub use register_044_0x2c_spcr::{ SPCR, SPCRBitField };
pub use register_045_0x2d_spsr::{ SPSR, SPSRBitField };
pub use register_046_0x2e_spdr::SPDR;
pub use register_048_0x30_acsr::{ ACSR, ACSRBitField };
pub use register_051_0x33_smcr::{ SMCR, SMCRBitField };
pub use register_052_0x34_mcusr::{ MCUSR, MCUSRBitField };
pub use register_053_0x35_mcucr::{ MCUCR, MCUCRBitField };
pub use register_055_0x37_spmcsr::{ SPMCSR, SPMCSRBitField };
pub use register_061_0x3d_spl::{ SPL, SPLBitField };
pub use register_062_0x3e_sph::{ SPH, SPHBitField };
pub use register_063_0x3f_sreg::{ SREG, SREGBitField };
pub use register_096_0x60_wdtcsr::{ WDTCSR, WDTCSRBitField };
pub use register_097_0x61_clkpr::{ CLKPR, CLKPRBitField };
pub use register_100_0x64_prr::{ PRR, PRRBitField };
pub use register_102_0x66_osccal::OSCCAL;
pub use register_104_0x68_pcicr::{ PCICR, PCICRBitField };

// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=278
mod register_107_0x6b_pcmsk0;
mod register_108_0x6c_pcmsk1;
mod register_109_0x6d_pcmsk2;
mod register_110_0x6e_timsk0;
mod register_111_0x6f_timsk1;
mod register_112_0x70_timsk2;
mod register_120_0x78_adcl;
mod register_121_0x79_adch;
mod register_122_0x7a_adcsra;
mod register_123_0x7b_adcsrb;
mod register_124_0x7c_admux;
mod register_126_0x7e_didr0;
mod register_127_0x7f_didr1;
mod register_128_0x80_tccr1a;
mod register_129_0x81_tccr1b;
mod register_130_0x82_tccr1c;
mod register_132_0x84_tcnt1l;
mod register_133_0x85_tcnt1h;
mod register_134_0x86_icr1l;
mod register_135_0x87_icr1h;
mod register_136_0x88_ocr1al;
mod register_137_0x89_ocr1ah;
mod register_138_0x8a_ocr1bl;
mod register_139_0x8b_ocr1bh;

pub use register_107_0x6b_pcmsk0::{ PCMSK0, PCMSK0BitField };
pub use register_108_0x6c_pcmsk1::{ PCMSK1, PCMSK1BitField };
pub use register_109_0x6d_pcmsk2::{ PCMSK2, PCMSK2BitField };
pub use register_110_0x6e_timsk0::{ TIMSK0, TIMSK0BitField };
pub use register_111_0x6f_timsk1::{ TIMSK1, TIMSK1BitField };
pub use register_112_0x70_timsk2::{ TIMSK2, TIMSK2BitField };
pub use register_120_0x78_adcl::ADCL;
pub use register_121_0x79_adch::ADCH;
pub use register_122_0x7a_adcsra::{ ADCSRA, ADCSRABitField };
pub use register_123_0x7b_adcsrb::{ ADCSRB, ADCSRBBitField };
pub use register_124_0x7c_admux::{ ADMUX, ADMUXBitField };
pub use register_126_0x7e_didr0::{ DIDR0, DIDR0BitField };
pub use register_127_0x7f_didr1::{ DIDR1, DIDR1BitField };
pub use register_128_0x80_tccr1a::{ TCCR1A, TCCR1ABitField };
pub use register_129_0x81_tccr1b::{ TCCR1B, TCCR1BBitField };
pub use register_130_0x82_tccr1c::{ TCCR1C, TCCR1CBitField };
pub use register_132_0x84_tcnt1l::TCNT1L;
pub use register_133_0x85_tcnt1h::TCNT1H;
pub use register_134_0x86_icr1l::ICR1L;
pub use register_135_0x87_icr1h::ICR1H;
pub use register_136_0x88_ocr1al::OCR1AL;
pub use register_137_0x89_ocr1ah::OCR1AH;
pub use register_138_0x8a_ocr1bl::OCR1BL;
pub use register_139_0x8b_ocr1bh::OCR1BH;

// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=277
mod register_176_0xb0_tccr2a;
mod register_177_0xb1_tccr2b;
mod register_178_0xb2_tcnt2;
mod register_179_0xb3_ocr2a;
mod register_180_0xb4_ocr2b;

pub use register_176_0xb0_tccr2a::{ TCCR2A, TCCR2ABitField };
pub use register_177_0xb1_tccr2b::{ TCCR2B, TCCR2BBitField };
pub use register_178_0xb2_tcnt2::TCNT2;
pub use register_179_0xb3_ocr2a::OCR2A;
pub use register_180_0xb4_ocr2b::OCR2B;

// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=276
mod register_182_0xb6_assr;
mod register_184_0xb8_twbr;
mod register_185_0xb9_twsr;
mod register_186_0xba_twar;
mod register_187_0xbb_twdr;
mod register_188_0xbc_twcr;
mod register_189_0xbd_twamr;
mod register_192_0xc0_ucsr0a;
mod register_193_0xc1_ucsr0b;
mod register_194_0xc2_ucsr0c;
mod register_196_0xc4_ubrr0l;
mod register_197_0xc5_ubrr0h;
mod register_198_0xc6_udr0;

pub use register_182_0xb6_assr::{ ASSR, ASSRBitField };
pub use register_184_0xb8_twbr::TWBR;
pub use register_185_0xb9_twsr::{ TWSR, TWSRBitField };
pub use register_186_0xba_twar::{ TWAR, TWARBitField };
pub use register_187_0xbb_twdr::TWDR;
pub use register_188_0xbc_twcr::{ TWCR, TWCRBitField };
pub use register_189_0xbd_twamr::{ TWAMR, TWAMRBitField };
pub use register_192_0xc0_ucsr0a::{ UCSR0A, UCSR0ABitField };
pub use register_193_0xc1_ucsr0b::{ UCSR0B, UCSR0BBitField };
pub use register_194_0xc2_ucsr0c::{ UCSR0C, UCSR0CBitField };
pub use register_196_0xc4_ubrr0l::UBRR0L;
pub use register_197_0xc5_ubrr0h::UBRR0H;
pub use register_198_0xc6_udr0::UDR0;

// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=275
// no registers
