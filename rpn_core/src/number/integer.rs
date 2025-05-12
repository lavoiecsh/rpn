use crate::number::{Number, NumberError};

macro_rules! impl_number_for_integer {
    ($T:ty) => {
        impl Number for $T {
            fn zero() -> Self { 0 }
            fn max() -> Self { Self::MAX }
            fn min() -> Self { Self::MIN }

            fn add(&self, other: &Self) -> Result<Self, NumberError> {
                self.checked_add(*other).ok_or(NumberError::Unchecked)
            }

            fn sub(&self, other: &Self) -> Result<Self, NumberError> {
                self.checked_sub(*other).ok_or(NumberError::Unchecked)
            }

            fn mul(&self, other: &Self) -> Result<Self, NumberError> {
                self.checked_mul(*other).ok_or(NumberError::Unchecked)
            }

            fn div(&self, other: &Self) -> Result<Self, NumberError> {
                if other == &Self::zero() {
                    return Err(NumberError::DivisionByZero);
                }
                self.checked_div(*other).ok_or(NumberError::Unchecked)
            }

            fn remainder(&self, other: &Self) -> Result<Self, NumberError> {
                if other == &Self::zero() {
                    return Err(NumberError::DivisionByZero);
                }
                self.checked_rem(*other).ok_or(NumberError::Unchecked)
            }
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
