use crate::number::Number;
use crate::operation::{OpStack, OperationError};
use crate::stack::Stack;

/// Subtracts first number on the stack from the second number and pushes back the result
pub fn subtract<S>(stack: OpStack<S>) -> Result<OpStack<S>, OperationError>
where
    S: Stack,
    S::Item: Number,
{
    stack.pop()?.pop()?.combine(Number::subtract)?.push()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn sub_errs_on_0_element_stack() {
        let stack = SmallStack::<i32>::default();
        let result = stack.evaluate(subtract);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn sub_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = stack.evaluate(subtract);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn sub_pushes_result_of_subtraction() {
        let stack = SmallStack::<i32>::two_elements(4, 1);
        let result = stack.evaluate(subtract);
        assert_matches!(result.unwrap().inspect(), (Some(3), None));
    }
}
