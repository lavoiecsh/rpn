use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Calculates the remained rof the first number on the stack divided by the second number and pushes back the result
/// ```
/// # use rpn_core::operation::{Operation, OperationError, Push, Remainder};
/// # use rpn_core::stack::{SmallStack, Stack};
/// let stack = SmallStack::<i32>::default()
///     .evaluate(Push(2))?
///     .evaluate(Push(5))?
///     .evaluate(Remainder)?;
/// assert_eq!(stack.size(), 1);
/// assert_eq!(stack.iter().next().unwrap(), &1);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Remainder;

impl<N: Number> Operation<N> for Remainder {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let c = a.remainder(&b)?;
        stack.push(c)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::NumberError;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn remainder_errs_on_empty_stack() {
        let stack = SmallStack::<i32>::default();
        let result = stack.evaluate(Remainder);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }
    
    #[test]
    fn remainder_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = stack.evaluate(Remainder);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }
    
    #[test]
    fn remainder_errs_on_division_by_zero() {
        let stack = SmallStack::<i32>::two_elements(0, 5);
        let result = stack.evaluate(Remainder);
        assert_matches!(result, Err(OperationError::Number(NumberError::DivisionByZero)));
    }
    
    #[test]
    fn remainder_pushes_remainder_of_division() {
        let stack = SmallStack::<i32>::two_elements(2, 5);
        let result = stack.evaluate(Remainder);
        assert_matches!(result.unwrap().inspect(), (Some(1), None));
    }
}