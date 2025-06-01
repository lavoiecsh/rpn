mod large;
mod small;

use core::error::Error;
use core::fmt::{Display, Formatter};
pub use large::LargeStack;
pub use small::SmallStack;

use crate::operation::{NoItems, OperationError, OperationStack};

pub trait Stack: Clone {
    type Item: Clone;
    
    fn size(&self) -> usize;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Self::Item>
    where
        Self::Item: 'a;
    fn push(&mut self, value: Self::Item) -> Result<(), StackError>;
    fn pop(&mut self) -> Result<Self::Item, StackError>;
    
    fn evaluate(&self, operation: impl FnOnce(OperationStack<Self, NoItems>) -> Result<OperationStack<Self, NoItems>, OperationError>) -> Result<Self, OperationError> {
        operation(OperationStack::new(self.clone())).map(OperationStack::stack)
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
