use crate::number::{Number, NumberError};
use crate::operation::OperationError;

macro_rules! impl_number_for_float {
    ($T:ty) => {
        impl Number for $T {
            const ZERO: Self = 0.;
            const ONE: Self = 1.;
            const TEN: Self = 10.;
            const MAX: Self = Self::MAX;
            const MIN: Self = Self::MIN;

            fn add(self, other: Self) -> Result<Self, OperationError> {
                let answer = self + other;
                if answer.is_infinite() {
                    Err(NumberError::Unchecked.into())
                } else {
                    Ok(answer)
                }
            }

            fn subtract(self, other: Self) -> Result<Self, OperationError> {
                let answer = self - other;
                if answer.is_infinite() {
                    Err(NumberError::Unchecked.into())
                } else {
                    Ok(answer)
                }
            }

            fn multiply(self, other: Self) -> Result<Self, OperationError> {
                let answer = self * other;
                if answer.is_infinite() {
                    Err(NumberError::Unchecked.into())
                } else {
                    Ok(answer)
                }
            }

            fn divide(self, other: Self) -> Result<Self, OperationError> {
                if other == Self::ZERO {
                    return Err(NumberError::DivisionByZero.into());
                }
                let answer = self / other;
                if answer.is_infinite() {
                    Err(NumberError::Unchecked.into())
                } else {
                    Ok(answer)
                }
            }
            
            fn remainder(self, other: Self) -> Result<Self, OperationError> {
                if other == Self::ZERO {
                    return Err(NumberError::DivisionByZero.into());
                }
                let answer = self % other;
                if answer.is_infinite() {
                    Err(NumberError::Unchecked.into())
                } else {
                    Ok(answer)
                }
            }
        }
    }
}

impl_number_for_float!(f32);
impl_number_for_float!(f64);