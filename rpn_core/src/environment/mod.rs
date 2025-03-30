pub mod large_stack;
pub mod small_stack;

use crate::number::Number;

pub trait Environment<T: Number> {
    fn stack_size(&self) -> usize;
    fn stack<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a;
    fn push(&mut self, value: T) -> Result<(), Error>;
    fn pop(&mut self) -> Result<T, Error>;
}

#[derive(Debug)]
pub enum Error {
    EmptyStack,
    StackSizeExceeded,
}
