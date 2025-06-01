use crate::operation::{OpStack, OperationError};
use crate::stack::Stack;

/// Rotates the top two elements of the stack
pub fn rotate<S>(stack: OpStack<S>) -> Result<OpStack<S>, OperationError> where S: Stack {
    stack.pop()?.pop()?.rotate()?.push()?.push()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn rotate_errs_on_empty_stack() {
        let stack = SmallStack::<i32>::default();
        let result = stack.evaluate(rotate);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }
    
    #[test]
    fn rotate_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = stack.evaluate(rotate);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }
    
    #[test]
    fn rotate_rotates_top_two_elements_of_stack() {
        let stack = SmallStack::<i32>::two_elements(1, 2);
        let result = stack.evaluate(rotate);
        assert_matches!(result.unwrap().inspect(), (Some(2), Some(1)));
    }
}