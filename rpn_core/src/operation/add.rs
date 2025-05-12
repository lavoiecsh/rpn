use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Add first two numbers on the stack and push back result
/// Does not modify the stack if it has less than 2 items
/// ```
/// # use rpn_core::operation::{Operation, OperationError, Push};
/// # use rpn_core::stack::{SmallStack, Stack};
/// # use rpn_core::operation::Add;
/// let mut stack = SmallStack::<i32>::default();
/// stack = Push(1).evaluate(&stack)?;
/// stack = Push(2).evaluate(&stack)?; 
/// stack = Add.evaluate(&stack)?;
/// assert_eq!(stack.size(), 1);
/// assert_eq!(stack.iter().next().unwrap(), &3);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Add;

impl<N: Number, S: Stack<N>> Operation<N, S> for Add {
    fn evaluate(self, stack: &S) -> Result<S, OperationError> {
        let mut stack = stack.clone();
        if stack.size() < 2 {
            return Err(OperationError::NotEnoughElements(2));
        }
        let a = stack.pop()?;
        let b = stack.pop()?;
        let c = a.add(&b)?;
        stack.push(c)?;
        Ok(stack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::NumberError;
    use crate::stack::SmallStack;
    use core::assert_matches::assert_matches;

    #[test]
    fn add_errs_on_0_element_stack() {
        let stack = SmallStack::<i32>::default();
        let result = Add.evaluate(&stack);
        assert_matches!(result, Err(OperationError::NotEnoughElements(2)));
    }

    #[test]
    fn add_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = Add.evaluate(&stack);
        assert_matches!(result, Err(OperationError::NotEnoughElements(2)));
    }

    #[test]
    fn add_pushes_result_of_addition() {
        let stack = SmallStack::<i32>::two_elements(1, 2);
        let new_stack = Add.evaluate(&stack).unwrap();
        assert_matches!(new_stack.inspect(), (Some(3), None));
    }

    #[test]
    fn add_errs_on_overflow() {
        let stack = SmallStack::<i8>::two_elements(i8::MAX, 2);
        let result = Add.evaluate(&stack);
        assert_matches!(result, Err(OperationError::Number(NumberError::Unchecked)));
    }
}
