use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Calculates the remained rof the first number on the stack divided by the second number and pushes back the result
/// ```
/// # use rpn_core::operation::{Operation, OperationError, Push, Remainder};
/// # use rpn_core::stack::{SmallStack, Stack};
/// let mut stack = SmallStack::<i32>::default();
/// stack = Push(2).evaluate(&stack)?;
/// stack = Push(5).evaluate(&stack)?;
/// stack = Remainder.evaluate(&stack)?;
/// assert_eq!(stack.size(), 1);
/// assert_eq!(stack.iter().next().unwrap(), &1);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Remainder;

impl<N: Number, S: Stack<N>> Operation<N, S> for Remainder {
    fn evaluate(self, stack: &S) -> Result<S, OperationError> {
        let mut stack = stack.clone();
        let a = stack.pop()?;
        let b = stack.pop()?;
        let c = a.remainder(&b)?;
        stack.push(c)?;
        Ok(stack)
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
        let result = Remainder.evaluate(&stack);
        assert_matches!(result, Err(OperationError::Stack(StackError::EmptyStack)));
    }
    
    #[test]
    fn remainder_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = Remainder.evaluate(&stack);
        assert_matches!(result, Err(OperationError::Stack(StackError::EmptyStack)));
    }
    
    #[test]
    fn remainder_errs_on_division_by_zero() {
        let stack = SmallStack::<i32>::two_elements(0, 5);
        let result = Remainder.evaluate(&stack);
        assert_matches!(result, Err(OperationError::Number(NumberError::DivisionByZero)));
    }
    
    #[test]
    fn remainder_pushes_remainder_of_division() {
        let stack = SmallStack::<i32>::two_elements(2, 5);
        let result = Remainder.evaluate(&stack);
        assert_matches!(result.unwrap().inspect(), (Some(1), None));
    }
}