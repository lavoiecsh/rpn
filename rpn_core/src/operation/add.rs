use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Add first two numbers on the stack and push back result
/// ```
/// # use rpn_core::operation::{Operation, OperationError, Push};
/// # use rpn_core::stack::{SmallStack, Stack};
/// # use rpn_core::operation::Add;
/// let stack = SmallStack::<i32>::default()
///     .evaluate(Push(1))?
///     .evaluate(Push(2))?
///     .evaluate(Add)?;
/// assert_eq!(stack.size(), 1);
/// assert_eq!(stack.iter().next().unwrap(), &3);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Add;

impl<N: Number> Operation<N> for Add {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        let c = a.add(&b)?;
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
    fn add_errs_on_0_element_stack() {
        let stack = SmallStack::<i32>::default();
        let result = stack.evaluate(Add);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn add_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = stack.evaluate(Add);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }

    #[test]
    fn add_pushes_result_of_addition() {
        let stack = SmallStack::<i32>::two_elements(1, 2);
        let new_stack = stack.evaluate(Add);
        assert_matches!(new_stack.unwrap().inspect(), (Some(3), None));
    }

    #[test]
    fn add_errs_on_overflow() {
        let stack = SmallStack::<i8>::two_elements(i8::MAX, 2);
        let result = stack.evaluate(Add);
        assert_matches!(result, Err(OperationError::Number(NumberError::Unchecked)));
    }
}
