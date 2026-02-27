use core::ops::{ BitOr, BitAnd, Shl };

pub trait Register
<
    DataType:
        BitOr<Output = DataType> + BitAnd<Output = DataType>
        + Shl<DataType, Output = DataType> + PartialEq<DataType>
>
{
    const ADDRESS: *mut DataType;
    
    fn get() -> DataType {
        unsafe {
            core::ptr::read_volatile(Self::ADDRESS)
        }
    }

    fn set(value: DataType) {
        unsafe {
            core::ptr::write_volatile(Self::ADDRESS, value);
        }
    }

    fn or(value: DataType) {
        Self::set(Self::get() | value);
    }

    fn and(value: DataType) {
        Self::set(Self::get() & value);
    }
}
