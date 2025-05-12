use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Multiplies the first two numbers on the stack and pushes back the result
/// ```
/// # use rpn_core::operation::{Multiply, Operation, OperationError, Push};
/// # use rpn_core::stack::{SmallStack, Stack};
/// let stack = SmallStack::<i32>::default()
///     .evaluate(Push(2))?
///     .evaluate(Push(3))?
///     .evaluate(Multiply)?;
/// assert_eq!(stack.size(), 1);
/// assert_eq!(stack.iter().next().unwrap(), &6);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Multiply;

impl<N: Number> Operation<N> for Multiply {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let c = a.multiply(&b)?;
        stack.push(c)?;
        Ok(())
    }
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
        let result = stack.evaluate(Multiply);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }
    
    #[test]
    fn mul_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = stack.evaluate(Multiply);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }
    
    #[test]
    fn mul_pushes_result_of_multiplication() {
        let stack = SmallStack::<i32>::two_elements(2, 3);
        let new_stack = stack.evaluate(Multiply);
        assert_matches!(new_stack.unwrap().inspect(), (Some(6), None));
    }
    
    #[test]
    fn mul_errs_on_overflow() {
        let stack = SmallStack::<i8>::two_elements(42, 100);
        let result = stack.evaluate(Multiply);
        assert_matches!(result, Err(OperationError::Number(NumberError::Unchecked)));
    }
}