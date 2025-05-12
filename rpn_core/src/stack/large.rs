use crate::number::Number;
use crate::stack::{Stack, StackError};

const MAX_STACK_SIZE: usize = 1024;

#[derive(Clone)]
pub struct LargeStack<N: Number> {
    stack: [N; MAX_STACK_SIZE],
    top: usize,
}

impl<N: Number> Default for LargeStack<N> {
    fn default() -> Self {
        Self {
            stack: [N::zero(); MAX_STACK_SIZE],
            top: 0,
        }
    }
}

impl<N: Number> Stack<N> for LargeStack<N> {
    fn size(&self) -> usize {
        self.top
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a N>
    where
        N: 'a,
    {
        self.stack.iter().take(self.top)
    }

    fn push(&mut self, value: N) -> Result<(), StackError> {
        if self.top == MAX_STACK_SIZE {
            Err(StackError::StackSizeExceeded(MAX_STACK_SIZE))
        } else {
            self.stack[self.top] = value;
            self.top += 1;
            Ok(())
        }
    }

    fn pop(&mut self) -> Result<N, StackError> {
        if self.top == 0 {
            Err(StackError::EmptyStack)
        } else {
            self.top -= 1;
            Ok(self.stack[self.top])
        }
    }
}
