use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Removes the last element from the stack
/// ```
/// # use rpn_core::operation::{Operation, OperationError, Pop, Push};
/// # use rpn_core::stack::{SmallStack, Stack};
/// let mut stack = SmallStack::<i32>::default();
/// stack = Push(3).evaluate(&stack)?;
/// stack = Pop.evaluate(&stack)?;
/// assert_eq!(stack.size(), 0);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Pop;

impl<N: Number, S: Stack<N>> Operation<N, S> for Pop {
    fn evaluate(self, stack: &S) -> Result<S, OperationError> {
        let mut stack = stack.clone();
        stack.pop()?;
        Ok(stack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn pop_errs_on_0_element_stack() {
        let stack = SmallStack::<i32>::default();
        let result = Pop.evaluate(&stack);
        assert_matches!(result, Err(OperationError::Stack(StackError::EmptyStack)));
    }
    
    #[test]
    fn pop_removes_last_element_from_stack() {
        let stack = SmallStack::<i32>::one_element(3);
        let new_stack = Pop.evaluate(&stack).unwrap();
        assert_matches!(new_stack.inspect(), (None, None));
    }
}
