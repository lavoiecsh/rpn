mod large;
mod small;

use core::error::Error;
use core::fmt::{Display, Formatter};
pub use large::LargeStack;
pub use small::SmallStack;

use crate::number::Number;
use crate::operation::{Operation, OperationError};

pub trait Stack<N: Number>: Clone {
    fn size(&self) -> usize;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a N>
    where
        N: 'a;
    fn push(&mut self, value: N) -> Result<(), StackError>;
    fn pop(&mut self) -> Result<N, StackError>;

    fn evaluate(&self, operation: impl Operation<N>) -> Result<Self, OperationError> {
        let mut stack = self.clone();
        operation.evaluate(&mut stack)?;
        Ok(stack)
    }
}

#[derive(Debug)]
pub enum StackError {
    Empty,
    SizeExceeded(usize),
}

impl Display for StackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            StackError::Empty => f.write_str("Empty stack"),
            StackError::SizeExceeded(size) => f.write_fmt(format_args!("Size Exceeded: {size}")),
        }
    }
}

impl Error for StackError {}
