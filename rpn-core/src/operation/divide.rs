use crate::number::Number;
use crate::operation::{OpStack, OperationError};
use crate::stack::Stack;

/// Divides the first number on the stack by the second number and pushes back the result
pub fn divide<S>(stack: OpStack<S>) -> Result<OpStack<S>, OperationError>
where
    S: Stack,
    S::Item: Number,
{
    stack.pop()?.pop()?.combine(Number::divide)?.push()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::NumberError;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn div_errs_on_0_element_stack() {
        let stack = SmallStack::<i32>::default();
        let result = stack.evaluate(divide);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn div_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = stack.evaluate(divide);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn div_errs_on_division_by_zero() {
        let stack = SmallStack::<i32>::two_elements(6, 0);
        let result = stack.evaluate(divide);
        assert_matches!(
            result,
            Err(OperationError::Number(NumberError::DivisionByZero))
        );
    }

    #[test]
    fn div_pushes_result_of_division() {
        let stack = SmallStack::<i32>::two_elements(6, 2);
        let result = stack.evaluate(divide);
        assert_matches!(result.unwrap().inspect(), (Some(3), None));
    }
}
