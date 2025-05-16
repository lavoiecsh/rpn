use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Subtracts first number on the stack from the second number and pushes back the result
/// ```
/// # use rpn_core::operation::{Operation, OperationError, Push, Subtract};
/// # use rpn_core::stack::{SmallStack, Stack};
/// let stack = SmallStack::<i32>::default()
///     .evaluate(Push(4))?
///     .evaluate(Push(1))?
///     .evaluate(Subtract)?;
/// assert_eq!(stack.size(), 1);
/// assert_eq!(stack.iter().next().unwrap(), &3);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Subtract;

impl<N: Number> Operation<N> for Subtract {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let c = b.subtract(&a)?;
        stack.push(c)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn sub_errs_on_0_element_stack() {
        let stack = SmallStack::<i32>::default();
        let result = stack.evaluate(Subtract);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn sub_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = stack.evaluate(Subtract);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn sub_pushes_result_of_subtraction() {
        let stack = SmallStack::<i32>::two_elements(4, 1);
        let result = stack.evaluate(Subtract);
        assert_matches!(result.unwrap().inspect(), (Some(3), None));
    }
}
