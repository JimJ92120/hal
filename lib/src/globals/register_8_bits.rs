pub type Address = *mut u8;

pub trait Register {
    const ADDRESS: Address;
    
    fn get() -> u8 {
        unsafe {
            core::ptr::read_volatile(Self::ADDRESS)
        }
    }

    fn set(value: u8) {
        unsafe {
            core::ptr::write_volatile(Self::ADDRESS, value);
        }
    }

    fn set_bit_mask(value: u8) {
        Self::set(Self::get() | (1 << value));
    }

    fn unset_bit_mask(value: u8) {
        Self::set(Self::get() & !(1 << value));
    }
}
