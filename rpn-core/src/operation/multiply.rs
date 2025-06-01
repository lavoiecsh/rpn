use crate::number::Number;
use crate::operation::{OpStack, OperationError};
use crate::stack::Stack;

/// Multiplies the first two numbers on the stack and pushes back the result
pub fn multiply<S>(stack: OpStack<S>) -> Result<OpStack<S>, OperationError>
where
    S: Stack,
    S::Item: Number,
{
    stack.pop()?.pop()?.combine(Number::multiply)?.push()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::NumberError;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn mul_errs_on_0_element_stack() {
        let stack = SmallStack::<i32>::default();
        let result = stack.evaluate(multiply);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn mul_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = stack.evaluate(multiply);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn mul_pushes_result_of_multiplication() {
        let stack = SmallStack::<i32>::two_elements(2, 3);
        let new_stack = stack.evaluate(multiply);
        assert_matches!(new_stack.unwrap().inspect(), (Some(6), None));
    }

    #[test]
    fn mul_errs_on_overflow() {
        let stack = SmallStack::<i8>::two_elements(42, 100);
        let result = stack.evaluate(multiply);
        assert_matches!(result, Err(OperationError::Number(NumberError::Unchecked)));
    }
}
