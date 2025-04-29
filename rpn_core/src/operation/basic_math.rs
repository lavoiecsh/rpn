use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

pub struct Add;

impl<N: Number> Operation<N> for Add {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError<N>> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let c = a + b;
        stack.push(c)?;
        Ok(())
    }
}

pub struct Sub;

impl<N: Number> Operation<N> for Sub {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError<N>> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let c = a - b;
        stack.push(c)?;
        Ok(())
    }
}

pub struct Mul;

impl<N: Number> Operation<N> for Mul {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError<N>> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let c = a * b;
        stack.push(c)?;
        Ok(())
    }
}

pub struct Div;

impl<N: Number> Operation<N> for Div {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError<N>> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let c = a / b;
        stack.push(c)?;
        Ok(())
    }
}
