use crate::number::Number;
use crate::operation::{Copy, Multiply, Operation, OperationError};
use crate::stack::Stack;

/// Squares last number on the stack
/// ```
/// # use rpn_core::operation::{OperationError, Push, Square};
/// # use rpn_core::stack::{SmallStack, Stack};
/// let stack = SmallStack::<i32>::default()
///     .evaluate(Push(3))?
///     .evaluate(Square)?;
/// assert_eq!(stack.size(), 1);
/// assert_eq!(stack.iter().next().unwrap(), &9);
/// # Ok::<(), OperationError>(())
/// ```
pub struct Square;

impl<N: Number> Operation<N> for Square {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError> {
        Copy.evaluate(stack)?;
        Multiply.evaluate(stack)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::{SmallStack, StackError};
    use core::assert_matches::assert_matches;

    #[test]
    fn square_errs_on_empty_stack() {
        let stack = SmallStack::<i32>::default();
        let result = stack.evaluate(Square);
        assert_matches!(result, Err(OperationError::Stack(StackError::Empty)));
    }
    
    #[test]
    fn square_squares_top_of_stack() {
        let stack = SmallStack::one_element(2);
        let result = stack.evaluate(Square);
        assert_matches!(result.unwrap().inspect(), (Some(4), None));
    }
}