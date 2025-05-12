use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Subtracts first number on the stack from the second number and pushes back the result
/// Does not modify the stack if it has less than 2 items
/// ```
/// # use rpn_core::operation::{Operation, OperationError, Push, Subtract};
/// # use rpn_core::stack::{SmallStack, Stack};
/// let mut stack = SmallStack::<i32>::default();
/// stack = Push(4).evaluate(&stack)?;
/// stack = Push(1).evaluate(&stack)?;
/// stack = Subtract.evaluate(&stack)?;
/// assert_eq!(stack.size(), 1);
/// assert_eq!(stack.iter().next().unwrap(), &3);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Subtract;

impl<N: Number, S: Stack<N>> Operation<N, S> for Subtract {
    fn evaluate(self, stack: &S) -> Result<S, OperationError> {
        let mut stack = stack.clone();
        let a = stack.pop()?;
        let b = stack.pop()?;
        let c = b.subtract(&a)?;
        stack.push(c)?;
        Ok(stack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn sub_errs_on_0_element_stack() {
        let stack = SmallStack::<i32>::default();
        let result = Subtract.evaluate(&stack);
        assert_matches!(result, Err(OperationError::Stack(StackError::EmptyStack)));
    }

    #[test]
    fn sub_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = Subtract.evaluate(&stack);
        assert_matches!(result, Err(OperationError::Stack(StackError::EmptyStack)));
    }

    #[test]
    fn sub_pushes_result_of_subtraction() {
        let stack = SmallStack::<i32>::two_elements(4, 1);
        let result = Subtract.evaluate(&stack).unwrap();
        assert_matches!(result.inspect(), (Some(3), None));
    }
}
