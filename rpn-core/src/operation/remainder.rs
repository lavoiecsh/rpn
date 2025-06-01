use crate::number::Number;
use crate::operation::{OpStack, OperationError};
use crate::stack::Stack;

/// Calculates the remained rof the first number on the stack divided by the second number and pushes back the result
pub fn remainder<S>(stack: OpStack<S>) -> Result<OpStack<S>, OperationError>
where
    S: Stack,
    S::Item: Number,
{
    stack.pop()?.pop()?.combine(Number::remainder)?.push()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::NumberError;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn remainder_errs_on_empty_stack() {
        let stack = SmallStack::<i32>::default();
        let result = stack.evaluate(remainder);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn remainder_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = stack.evaluate(remainder);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn remainder_errs_on_division_by_zero() {
        let stack = SmallStack::<i32>::two_elements(5, 0);
        let result = stack.evaluate(remainder);
        assert_matches!(
            result,
            Err(OperationError::Number(NumberError::DivisionByZero))
        );
    }

    #[test]
    fn remainder_pushes_remainder_of_division() {
        let stack = SmallStack::<i32>::two_elements(5, 2);
        let result = stack.evaluate(remainder);
        assert_matches!(result.unwrap().inspect(), (Some(1), None));
    }
}
