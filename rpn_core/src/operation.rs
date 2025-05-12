use crate::number::{Number, NumberError};
use crate::stack::{Stack, StackError};

mod add;
mod div;
mod mul;
mod pop;
mod push;
mod sub;
mod copy;

pub use add::Add;
pub use copy::Copy;
pub use div::Div;
pub use mul::Mul;
pub use pop::Pop;
pub use push::Push;
pub use sub::Sub;

pub trait Operation<N: Number, S: Stack<N>> {
    fn evaluate(self, stack: &S) -> Result<S, OperationError>;
}

#[derive(Debug)]
pub enum OperationError {
    NotEnoughElements(usize),
    Stack(StackError),
    Number(NumberError),
}

impl From<StackError> for OperationError {
    fn from(value: StackError) -> Self {
        Self::Stack(value)
    }
}

impl From<NumberError> for OperationError {
    fn from(value: NumberError) -> Self {
        Self::Number(value)
    }
}