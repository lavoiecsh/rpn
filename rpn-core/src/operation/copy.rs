use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Copies the top of the stack
/// ```
/// # use rpn_core::operation::{Copy, Operation, OperationError, Push};
/// # use rpn_core::stack::{SmallStack, Stack};
/// let stack = SmallStack::<i32>::default()
///     .evaluate(Push(1))?
///     .evaluate(Copy)?;
/// assert_eq!(stack.size(), 2);
/// assert!(stack.iter().all(|n| n == &1));
/// # Ok::<(), OperationError>(())
/// ```
pub struct Copy;

impl<N: Number> Operation<N> for Copy {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError> {
        let n = stack.pop()?;
        stack.push(n)?;
        stack.push(n)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn copy_errs_on_empty_stack() {
        let stack = SmallStack::<i32>::default();
        let result = stack.evaluate(Copy);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn copy_errs_on_full_stack() {
        let stack = SmallStack::<i32>::two_elements(1, 2);
        let result = stack.evaluate(Copy);
        assert_matches!(result, Err(OperationError::Stack(StackError::SizeExceeded(2))));
    }

    #[test]
    fn copy_copies_top_element_of_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = stack.evaluate(Copy);
        assert_matches!(result.unwrap().inspect(), (Some(1), Some(1)));
    }
}