use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

pub struct Push<N: Number> {
    value: N,
}

impl<N: Number> Push<N> {
    pub fn new(value: N) -> Self {
        Self { value }
    }
}

impl<N: Number> Operation<N> for Push<N> {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError<N>> {
        stack.push(self.value)?;
        Ok(())
    }
}

pub struct Pop;

impl<N: Number> Operation<N> for Pop {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError<N>> {
        stack.pop()?;
        Ok(())
    }
}