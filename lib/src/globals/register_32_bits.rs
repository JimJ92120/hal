pub type Address = *mut u32;

pub trait Register {
    const ADDRESS: Address;
    
    fn get() -> u32 {
        unsafe {
            core::ptr::read_volatile(Self::ADDRESS)
        }
    }

    fn set(value: u32) {
        unsafe {
            core::ptr::write_volatile(Self::ADDRESS, value);
        }
    }

    fn clear() {
        Self::set(0);
    }

    fn set_bit_mask(value: u32) {
        Self::set(Self::get() | (1 << value));
    }

    fn unset_bit_mask(value: u32) {
        Self::set(Self::get() & !(1 << value));
    }

    fn is_bit_mask_set(value: u32) -> bool {
        0 != Self::get() & (1 << value)
    }
}
