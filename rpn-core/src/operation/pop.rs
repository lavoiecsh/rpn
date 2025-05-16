use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Removes the last element from the stack
/// ```
/// # use rpn_core::operation::{Operation, OperationError, Pop, Push};
/// # use rpn_core::stack::{SmallStack, Stack};
/// let mut stack = SmallStack::<i32>::default();
/// stack = stack.evaluate(Push(3))?;
/// stack = stack.evaluate(Pop)?;
/// assert_eq!(stack.size(), 0);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Pop;

impl<N: Number> Operation<N> for Pop {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError> {
        stack.pop()?;
        Ok(())
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
        let result = stack.evaluate(Pop);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }
    
    #[test]
    fn pop_removes_last_element_from_stack() {
        let stack = SmallStack::<i32>::one_element(3);
        let new_stack = stack.evaluate(Pop);
        assert_matches!(new_stack.unwrap().inspect(), (None, None));
    }
}
