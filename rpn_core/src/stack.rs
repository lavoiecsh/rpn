mod large;
mod small;

pub use large::LargeStack;
pub use small::SmallStack;

use crate::number::Number;

pub trait Stack<N: Number>: Clone {
    fn size(&self) -> usize;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a N> where N: 'a;
    fn push(&mut self, value: N) -> Result<(), StackError>;
    fn pop(&mut self) -> Result<N, StackError>;
}

#[derive(Debug)]
pub enum StackError {
    EmptyStack,
    StackSizeExceeded(usize),
}
