use crate::number::Number;
use crate::stack::{Stack, StackError};

pub struct SmallStack<N: Number> {
    a: Option<N>,
    b: Option<N>,
}

impl<N: Number> SmallStack<N> {
    pub fn new() -> Self {
        Self { a: None, b: None }
    }
}

impl<N: Number> Stack<N> for SmallStack<N> {
    fn size(&self) -> usize {
        match (self.a, self.b) {
            (None, None) => 0,
            (Some(_), None) => 1,
            (Some(_), Some(_)) => 2,
            (None, Some(_b)) => {
                unreachable!("invalid state within SmallStackEnvironment: (None, {_b:?})")
            }
        }
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a N>
    where
        N: 'a,
    {
        StackIterator {
            a: &self.a,
            b: &self.b,
        }
    }

    fn push(&mut self, value: N) -> Result<(), StackError> {
        match (self.a, self.b) {
            (None, None) => {
                self.a = Some(value);
                Ok(())
            }
            (Some(_), None) => {
                self.b = Some(value);
                Ok(())
            }
            (Some(_), Some(_)) => Err(StackError::StackSizeExceeded(2)),
            (None, Some(_b)) => {
                unreachable!("invalid state within SmallStackEnvironment: (None, {_b:?})")
            }
        }
    }

    fn pop(&mut self) -> Result<N, StackError> {
        match (self.a, self.b) {
            (None, None) => Err(StackError::EmptyStack),
            (Some(a), None) => {
                self.a = None;
                Ok(a)
            }
            (Some(_), Some(b)) => {
                self.b = None;
                Ok(b)
            }
            (None, Some(_b)) => {
                unreachable!("invalid state within SmallStackEnvironment: (None, {_b:?})")
            }
        }
    }
}

struct StackIterator<'a, N: Number> {
    a: &'a Option<N>,
    b: &'a Option<N>,
}

impl<'a, N: Number> Iterator for StackIterator<'a, N> {
    type Item = &'a N;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a, self.b) {
            (None, None) => None,
            (Some(a), _) => {
                self.a = self.b;
                self.b = &None;
                Some(a)
            }
            (None, Some(_b)) => {
                unreachable!("invalid state within SmallStackEnvironment: (None, {_b:?})")
            }
        }
    }
}
