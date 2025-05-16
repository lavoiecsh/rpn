use crate::number::{Number, NumberError};

macro_rules! impl_number_for_float {
    ($T:ty) => {
        impl Number for $T {
            fn zero() -> Self { 0. }
            fn max() -> Self { Self::MAX }
            fn min() -> Self { Self::MIN }

            fn add(&self, other: &Self) -> Result<Self, NumberError> {
                let answer = *self + *other;
                if answer.is_infinite() {
                    Err(NumberError::Unchecked)
                } else {
                    Ok(answer)
                }
            }

            fn subtract(&self, other: &Self) -> Result<Self, NumberError> {
                let answer = *self - *other;
                if answer.is_infinite() {
                    Err(NumberError::Unchecked)
                } else {
                    Ok(answer)
                }
            }

            fn multiply(&self, other: &Self) -> Result<Self, NumberError> {
                let answer = *self * *other;
                if answer.is_infinite() {
                    Err(NumberError::Unchecked)
                } else {
                    Ok(answer)
                }
            }

            fn divide(&self, other: &Self) -> Result<Self, NumberError> {
                if other == &Self::zero() {
                    return Err(NumberError::DivisionByZero);
                }
                let answer = *self / *other;
                if answer.is_infinite() {
                    Err(NumberError::Unchecked)
                } else {
                    Ok(answer)
                }
            }
            
            fn remainder(&self, other: &Self) -> Result<Self, NumberError> {
                if other == &Self::zero() {
                    return Err(NumberError::DivisionByZero);
                }
                let answer = *self % *other;
                if answer.is_infinite() {
                    Err(NumberError::Unchecked)
                } else {
                    Ok(answer)
                }
            }
        }
    }
}

impl_number_for_float!(f32);
impl_number_for_float!(f64);