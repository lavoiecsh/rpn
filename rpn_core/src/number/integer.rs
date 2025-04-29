use crate::number::Number;

macro_rules! impl_number_for_integer {
    ($T:ty) => {
        impl Number for $T {
            fn zero() -> Self { 0 }
        }
    }
}

impl_number_for_integer!(u8);
impl_number_for_integer!(u16);
impl_number_for_integer!(u32);
impl_number_for_integer!(u64);
impl_number_for_integer!(u128);
impl_number_for_integer!(i8);
impl_number_for_integer!(i16);
impl_number_for_integer!(i32);
impl_number_for_integer!(i64);
impl_number_for_integer!(i128);
