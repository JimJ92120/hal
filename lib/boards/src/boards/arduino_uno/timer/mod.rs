use core::ops::{ BitOr, BitAnd, Shl };

mod timer_0;
mod timer_1;
mod timer_2;

pub use timer_0::Timer0;
pub use timer_1::Timer1;
pub use timer_2::Timer2;

pub trait Timer
<
    DataType:
        BitOr<Output = DataType> + BitAnd<Output = DataType>
        + Shl<DataType, Output = DataType> + PartialEq<DataType>
>
{
    fn init();
    fn set_a(value: DataType);
    fn set_b(value: DataType);
}
