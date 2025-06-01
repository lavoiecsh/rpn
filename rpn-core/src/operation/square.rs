use crate::number::Number;
use crate::operation::{copy, multiply, OpStack, OperationError};
use crate::stack::Stack;

/// Squares last number on the stack
pub fn square<S>(stack: OpStack<S>) -> Result<OpStack<S>, OperationError> where S: Stack, S::Item: Number {
    Ok(stack).and_then(copy).and_then(multiply)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn square_errs_on_empty_stack() {
        let stack = SmallStack::<i32>::default();
        let result = stack.evaluate(square);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }
    
    #[test]
    fn square_squares_top_of_stack() {
        let stack = SmallStack::one_element(2);
        let result = stack.evaluate(square);
        assert_matches!(result.unwrap().inspect(), (Some(4), None));
    }
}