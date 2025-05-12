use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Multiplies the first two numbers on the stack and pushes back the result
/// Does not modify the stack if it has less than 2 items
/// ```
/// # use rpn_core::operation::{Multiply, Operation, OperationError, Push};
/// # use rpn_core::stack::{SmallStack, Stack};
/// let mut stack = SmallStack::<i32>::default();
/// stack = Push(2).evaluate(&stack)?;
/// stack = Push(3).evaluate(&stack)?;
/// stack = Multiply.evaluate(&stack)?;
/// assert_eq!(stack.size(), 1);
/// assert_eq!(stack.iter().next().unwrap(), &6);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Multiply;

impl<N: Number, S: Stack<N>> Operation<N, S> for Multiply {
    fn evaluate(self, stack: &S) -> Result<S, OperationError> {
        if stack.size() < 2 {
            return Err(OperationError::NotEnoughElements(2));
        }
        let mut stack = stack.clone();
        let a = stack.pop()?;
        let b = stack.pop()?;
        let c = a.multiply(&b)?;
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
    fn mul_errs_on_0_element_stack() {
        let stack = SmallStack::<i32>::default();
        let result = Multiply.evaluate(&stack);
        assert_matches!(result, Err(OperationError::NotEnoughElements(2)));
    }
    
    #[test]
    fn mul_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = Multiply.evaluate(&stack);
        assert_matches!(result, Err(OperationError::NotEnoughElements(2)));
    }
    
    #[test]
    fn mul_pushes_result_of_multiplication() {
        let stack = SmallStack::<i32>::two_elements(2, 3);
        let new_stack = Multiply.evaluate(&stack).unwrap();
        assert_matches!(new_stack.inspect(), (Some(6), None));
    }
    
    #[test]
    fn mul_errs_on_overflow() {
        let stack = SmallStack::<i8>::two_elements(42, 100);
        let result = Multiply.evaluate(&stack);
        assert_matches!(result, Err(OperationError::Number(NumberError::Unchecked)));
    }
}