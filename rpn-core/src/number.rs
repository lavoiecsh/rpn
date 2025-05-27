mod float;
mod integer;

use core::error::Error;
use core::fmt::{Debug, Display, Formatter};

pub trait Number: Sized + Copy + Clone + Debug + PartialOrd {
    const ZERO: Self;
    const ONE: Self;
    const TEN: Self;
    const MIN: Self;
    const MAX: Self;

    fn add(&self, other: &Self) -> Result<Self, NumberError>;
    fn subtract(&self, other: &Self) -> Result<Self, NumberError>;
    fn multiply(&self, other: &Self) -> Result<Self, NumberError>;
    fn divide(&self, other: &Self) -> Result<Self, NumberError>;
    fn remainder(&self, other: &Self) -> Result<Self, NumberError>;
}

#[derive(Debug)]
pub enum NumberError {
    Unchecked,
    DivisionByZero,
}

impl Display for NumberError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            NumberError::Unchecked => f.write_str("Unchecked number error"),
            NumberError::DivisionByZero => f.write_str("Division by zero error"),
        }
    }
}

impl Error for NumberError {}
