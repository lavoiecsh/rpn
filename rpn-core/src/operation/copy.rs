use crate::operation::{OpStack, OperationError};
use crate::stack::Stack;

/// Copies the top of the stack
pub fn copy<S>(stack: OpStack<S>) -> Result<OpStack<S>, OperationError>
where
    S: Stack,
    S::Item: Copy,
{
    stack.pop()?.copy()?.push()?.push()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn copy_errs_on_empty_stack() {
        let stack = SmallStack::<i32>::default();
        let result = stack.evaluate(copy);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn copy_errs_on_full_stack() {
        let stack = SmallStack::<i32>::two_elements(1, 2);
        let result = stack.evaluate(copy);
        assert_matches!(
            result,
            Err(OperationError::Stack(StackError::SizeExceeded(2)))
        );
    }

    #[test]
    fn copy_copies_top_element_of_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = stack.evaluate(copy);
        assert_matches!(result.unwrap().inspect(), (Some(1), Some(1)));
    }
}
