use crate::number::Number;

macro_rules! impl_number_for_float {
    ($T:ty) => {
        impl Number for $T {
            fn zero() -> Self { 0. }
        }
    }
}

impl_number_for_float!(f32);
impl_number_for_float!(f64);