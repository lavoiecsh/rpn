use crate::stack::{Stack, StackError};

const MAX_STACK_SIZE: usize = 1024;

#[derive(Clone)]
pub struct LargeStack<N> {
    stack: [N; MAX_STACK_SIZE],
    top: usize,
}

impl<N: Default + Copy> Default for LargeStack<N> {
    fn default() -> Self {
        Self {
            stack: [N::default(); MAX_STACK_SIZE],
            top: 0,
        }
    }
}

impl<N: Copy> Stack for LargeStack<N> {
    type Item = N;
    
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
            Err(StackError::SizeExceeded(MAX_STACK_SIZE))
        } else {
            self.stack[self.top] = value;
            self.top += 1;
            Ok(())
        }
    }

    fn pop(&mut self) -> Result<N, StackError> {
        if self.top == 0 {
            Err(StackError::Empty)
        } else {
            self.top -= 1;
            Ok(self.stack[self.top])
        }
    }
}
