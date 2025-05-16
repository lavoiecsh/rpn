mod integer;
mod float;

use core::fmt::{Debug, Display};

pub trait Number:
Sized
+ Copy
+ Clone
+ Debug
+ Display
+ PartialOrd
{
    fn zero() -> Self;
    fn max() -> Self;
    fn min() -> Self;
    
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