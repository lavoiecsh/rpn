use core::error::Error;
use core::fmt::{Display, Formatter};
use crate::number::{Number, NumberError};
use crate::stack::{Stack, StackError};

mod add;
mod copy;
mod divide;
mod multiply;
mod pop;
mod push;
mod remainder;
mod rotate;
mod square;
mod subtract;

pub use add::Add;
pub use copy::Copy;
pub use divide::Divide;
pub use multiply::Multiply;
pub use pop::Pop;
pub use push::Push;
pub use remainder::Remainder;
pub use rotate::Rotate;
pub use square::Square;
pub use subtract::Subtract;

pub trait Operation<N: Number> {
    fn evaluate(&self, stack: &mut impl Stack<N>) -> Result<(), OperationError>;
}

#[derive(Debug)]
pub enum OperationError {
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

impl Display for OperationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            OperationError::Stack(ref e) => f.write_fmt(format_args!("Stack error: {e}")),
            OperationError::Number(ref e) => f.write_fmt(format_args!("Number error: {e}")),
        }
    }
}

impl Error for OperationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            OperationError::Stack(ref e) => Some(e),
            OperationError::Number(ref e) => Some(e),
        }
    }
}