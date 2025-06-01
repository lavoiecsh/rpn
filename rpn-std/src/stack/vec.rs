use rpn_core::stack::{Stack, StackError};

#[derive(Clone)]
pub struct VecStack<N: Clone> {
    stack: Vec<N>,
}

impl<N: Clone> Default for VecStack<N> {
    fn default() -> Self {
        Self { stack: Vec::new() }
    }
}

impl<N: Clone> Stack for VecStack<N> {
    type Item = N;
    
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