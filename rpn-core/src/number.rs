mod float;
mod integer;

use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use crate::operation::OperationError;

pub trait Number: Sized + Copy + Clone + Debug + PartialOrd {
    const ZERO: Self;
    const ONE: Self;
    const TEN: Self;
    const MIN: Self;
    const MAX: Self;

    fn add(self, other: Self) -> Result<Self, OperationError>;
    fn subtract(self, other: Self) -> Result<Self, OperationError>;
    fn multiply(self, other: Self) -> Result<Self, OperationError>;
    fn divide(self, other: Self) -> Result<Self, OperationError>;
    fn remainder(self, other: Self) -> Result<Self, OperationError>;
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
