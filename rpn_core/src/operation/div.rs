use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Divides the first number on the stack by the second number and pushes back the result
/// Does not modify the stack if it has less than 2 items
/// ```
/// # use rpn_core::operation::{Div, Operation, OperationError, Push};
/// # use rpn_core::stack::{SmallStack, Stack};
/// let mut stack = SmallStack::<i32>::default();
/// stack = Push(2).evaluate(&stack)?;
/// stack = Push(6).evaluate(&stack)?;
/// stack = Div.evaluate(&stack)?;
/// assert_eq!(stack.size(), 1);
/// assert_eq!(stack.iter().next().unwrap(), &3);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Div;

impl<N: Number, S: Stack<N>> Operation<N, S> for Div {
    fn evaluate(self, stack: &S) -> Result<S, OperationError> {
        let mut stack = stack.clone();
        let a = stack.pop()?;
        let b = stack.pop()?;
        let c = a.div(&b)?;
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
    fn div_errs_on_0_element_stack() {
        let stack = SmallStack::<i32>::default();
        let result = Div.evaluate(&stack);
        assert_matches!(result, Err(OperationError::Stack(StackError::EmptyStack)));
    }

    #[test]
    fn div_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = Div.evaluate(&stack);
        assert_matches!(result, Err(OperationError::Stack(StackError::EmptyStack)));
    }

    #[test]
    fn div_errs_on_division_by_zero() {
        let stack = SmallStack::<i32>::two_elements(0, 6);
        let result = Div.evaluate(&stack);
        assert_matches!(result, Err(OperationError::Number(NumberError::DivisionByZero)));
    }

    #[test]
    fn div_pushes_result_of_division() {
        let stack = SmallStack::<i32>::two_elements(2, 6);
        let result = Div.evaluate(&stack).unwrap();
        assert_matches!(result.inspect(), (Some(3), None));
    }
}