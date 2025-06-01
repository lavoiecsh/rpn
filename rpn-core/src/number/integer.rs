use crate::number::{Number,NumberError};
use crate::operation::OperationError;

macro_rules! impl_number_for_integer {
    ($T:ty) => {
        impl Number for $T {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const TEN: Self = 10;
            const MAX: Self = Self::MAX;
            const MIN: Self = Self::MIN;

            fn add(self, other: Self) -> Result<Self, OperationError> {
                self.checked_add(other).ok_or(NumberError::Unchecked.into())
            }

            fn subtract(self, other: Self) -> Result<Self, OperationError> {
                self.checked_sub(other).ok_or(NumberError::Unchecked.into())
            }

            fn multiply(self, other: Self) -> Result<Self, OperationError> {
                self.checked_mul(other).ok_or(NumberError::Unchecked.into())
            }

            fn divide(self, other: Self) -> Result<Self, OperationError> {
                if other == Self::ZERO {
                    return Err(NumberError::DivisionByZero.into());
                }
                self.checked_div(other).ok_or(NumberError::Unchecked.into())
            }

            fn remainder(self, other: Self) -> Result<Self, OperationError> {
                if other == Self::ZERO {
                    return Err(NumberError::DivisionByZero.into());
                }
                self.checked_rem(other).ok_or(NumberError::Unchecked.into())
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