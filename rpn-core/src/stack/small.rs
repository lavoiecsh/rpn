use core::fmt::Debug;
use crate::stack::{Stack, StackError};

#[derive(Clone, Debug)]
pub struct SmallStack<N> {
    a: Option<N>,
    b: Option<N>,
}

impl<N> Default for SmallStack<N> {
    fn default() -> Self {
        Self { a: None, b: None }
    }
}

#[cfg(test)]
impl<N: Copy> SmallStack<N> {
    pub(crate) fn empty() -> Self {
        Self { a: None, b: None }
    }

    pub(crate) fn one_element(a: N) -> Self {
        Self { a: Some(a), b: None }
    }
    
    pub(crate) fn two_elements(a: N, b: N) -> Self {
        Self { a: Some(a), b: Some(b) }
    }
    
    pub(crate) fn inspect(&self) -> (Option<N>, Option<N>) {
        (self.a, self.b)
    }
}

impl<N: Copy + Debug> Stack for SmallStack<N> {
    type Item = N;
    
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
            (Some(_), Some(_)) => Err(StackError::SizeExceeded(2)),
            (None, Some(_b)) => {
                unreachable!("invalid state within SmallStackEnvironment: (None, {_b:?})")
            }
        }
    }

    fn pop(&mut self) -> Result<N, StackError> {
        match (self.a, self.b) {
            (None, None) => Err(StackError::Empty),
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

struct StackIterator<'a, N> {
    a: &'a Option<N>,
    b: &'a Option<N>,
}

impl<'a, N: Debug> Iterator for StackIterator<'a, N> {
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
