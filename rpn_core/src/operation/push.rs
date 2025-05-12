use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Pushes the number on top of the stack
/// ```
/// # use rpn_core::operation::{Operation, OperationError, Push};
/// # use rpn_core::stack::{SmallStack, Stack};
/// let mut stack = SmallStack::<i32>::default();
/// stack = Push(1).evaluate(&stack)?;
/// assert_eq!(stack.size(), 1);
/// assert_eq!(stack.iter().next().unwrap(), &1);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Push<N: Number>(pub N);

impl<N: Number, S: Stack<N>> Operation<N, S> for Push<N> {
    fn evaluate(self, stack: &S) -> Result<S, OperationError> {
        let mut stack = stack.clone();
        stack.push(self.0)?;
        Ok(stack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn push_pushes_element_on_top_of_stack() {
        let stack = SmallStack::<i32>::default();
        let result = Push(3).evaluate(&stack).unwrap();
        assert_matches!(result.inspect(), (Some(3), None));
    }
    
    #[test]
    fn push_returns_error_on_full_stack() {
        let stack = SmallStack::<i32>::two_elements(1, 2);
        let result = Push(3).evaluate(&stack);
        assert_matches!(result, Err(OperationError::Stack(StackError::StackSizeExceeded(2))));
    }
}
