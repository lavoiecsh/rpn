use crate::number::Number;
use crate::operation::{OpStack, OperationError};
use crate::stack::Stack;

/// Adds first two numbers on the stack and pushes back result
pub fn add<S>(stack: OpStack<S>) -> Result<OpStack<S>, OperationError>
where
    S: Stack,
    S::Item: Number,
{
    stack.pop()?.pop()?.combine(Number::add)?.push()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::NumberError;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn add_errs_on_0_element_stack() {
        let stack = SmallStack::<i32>::default();
        let result = stack.evaluate(add);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn add_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = stack.evaluate(add);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn add_pushes_result_of_addition() {
        let stack = SmallStack::<i32>::two_elements(1, 2);
        let new_stack = stack.evaluate(add);
        assert_matches!(new_stack.unwrap().inspect(), (Some(3), None));
    }

    #[test]
    fn add_errs_on_overflow() {
        let stack = SmallStack::<i8>::two_elements(i8::MAX, 2);
        let result = stack.evaluate(add);
        assert_matches!(result, Err(OperationError::Number(NumberError::Unchecked)));
    }
}
