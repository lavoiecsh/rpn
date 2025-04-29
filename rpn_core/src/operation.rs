use crate::number::Number;
use crate::stack::{Stack, StackError};

pub mod basic_math;
pub mod stack_manipulation;

pub trait Operation<N: Number> {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError<N>>;
}

#[derive(Debug)]
pub enum OperationError<N: Number> {
    Overflow(N),
    Underflow(N),
    Stack(StackError),
}

impl<N: Number> From<StackError> for OperationError<N> {
    fn from(value: StackError) -> Self {
        Self::Stack(value)
    }
}