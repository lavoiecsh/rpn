use rpn_core::number::Number;
use rpn_core::stack::{Stack, StackError};

#[derive(Clone)]
pub struct VecStack<N: Number> {
    stack: Vec<N>,
}

impl<N: Number> Default for VecStack<N> {
    fn default() -> Self {
        Self { stack: Vec::new() }
    }
}

impl<N: Number> Stack<N> for VecStack<N> {
    fn size(&self) -> usize {
        self.stack.len()
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item=&'a N>
    where
        N: 'a
    {
        self.stack.iter()
    }

    fn push(&mut self, value: N) -> Result<(), StackError> {
        self.stack.push(value);
        Ok(())
    }

    fn pop(&mut self) -> Result<N, StackError> {
        self.stack.pop()
            .ok_or(StackError::Empty)
    }
}