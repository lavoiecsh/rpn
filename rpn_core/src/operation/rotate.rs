use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Rotates the top two elements of the stack
/// ```
/// # use rpn_core::operation::{Operation, OperationError, Push, Rotate};
/// # use rpn_core::stack::{SmallStack, Stack};
/// let mut stack = SmallStack::<i32>::default();
/// stack = Push(1).evaluate(&stack)?;
/// stack = Push(2).evaluate(&stack)?;
/// stack = Rotate.evaluate(&stack)?;
/// assert_eq!(stack.size(), 2);
/// let mut it = stack.iter();
/// assert_eq!(it.next().unwrap(), &2);
/// assert_eq!(it.next().unwrap(), &1);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Rotate;

impl<N: Number, S: Stack<N>> Operation<N, S> for Rotate {
    fn evaluate(self, stack: &S) -> Result<S, OperationError> {
        let mut stack = stack.clone();
        let a = stack.pop()?;
        let b = stack.pop()?;
        stack.push(a)?;
        stack.push(b)?;
        Ok(stack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn rotate_errs_on_empty_stack() {
        let stack = SmallStack::<i32>::default();
        let result = Rotate.evaluate(&stack);
        assert_matches!(result, Err(OperationError::Stack(StackError::EmptyStack)));
    }
    
    #[test]
    fn rotate_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = Rotate.evaluate(&stack);
        assert_matches!(result, Err(OperationError::Stack(StackError::EmptyStack)));
    }
    
    #[test]
    fn rotate_rotates_top_two_elements_of_stack() {
        let stack = SmallStack::<i32>::two_elements(1, 2);
        let result = Rotate.evaluate(&stack).unwrap();
        assert_matches!(result.inspect(), (Some(2), Some(1)));
    }
}