use crate::number::Number;
use crate::operation::{Operation, OperationError};
use crate::stack::Stack;

/// Rotates the top two elements of the stack
/// ```
/// # use rpn_core::operation::{Operation, OperationError, Push, Rotate};
/// # use rpn_core::stack::{SmallStack, Stack};
/// let stack = SmallStack::<i32>::default()
///     .evaluate(Push(1))?
///     .evaluate(Push(2))?
///     .evaluate(Rotate)?;
/// assert_eq!(stack.size(), 2);
/// let mut it = stack.iter();
/// assert_eq!(it.next().unwrap(), &2);
/// assert_eq!(it.next().unwrap(), &1);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Rotate;

impl<N: Number> Operation<N> for Rotate {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        stack.push(a)?;
        stack.push(b)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn rotate_errs_on_empty_stack() {
        let stack = SmallStack::<i32>::default();
        let result = stack.evaluate(Rotate);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }
    
    #[test]
    fn rotate_errs_on_1_element_stack() {
        let stack = SmallStack::<i32>::one_element(1);
        let result = stack.evaluate(Rotate);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }
    
    #[test]
    fn rotate_rotates_top_two_elements_of_stack() {
        let stack = SmallStack::<i32>::two_elements(1, 2);
        let result = stack.evaluate(Rotate);
        assert_matches!(result.unwrap().inspect(), (Some(2), Some(1)));
    }
}