use crate::environment;
use crate::environment::Environment;
use crate::number::Number;

const MAX_STACK_SIZE: usize = 1024;

pub struct LargeStackEnvironment<T: Number> {
    stack: [T; MAX_STACK_SIZE],
    top: usize,
}

impl<T: Number> LargeStackEnvironment<T> {
    pub fn new() -> Self {
        Self {
            stack: [T::zero(); MAX_STACK_SIZE],
            top: 0,
        }
    }
}

impl<T: Number> Environment<T> for LargeStackEnvironment<T> {
    fn stack_size(&self) -> usize {
        self.top
    }

    fn stack<'a>(&'a self) -> impl Iterator<Item=&'a T> where T: 'a {
        self.stack.iter().take(self.top)
    }

    fn push(&mut self, value: T) -> Result<(), environment::Error> {
        if self.top == MAX_STACK_SIZE {
            Err(environment::Error::StackSizeExceeded)
        } else {
            self.stack[self.top] = value;
            self.top += 1;
            Ok(())
        }
    }

    fn pop(&mut self) -> Result<T, environment::Error> {
        if self.top == 0 {
            Err(environment::Error::EmptyStack)
        } else {
            self.top -= 1;
            Ok(self.stack[self.top])
        }
    }
}
