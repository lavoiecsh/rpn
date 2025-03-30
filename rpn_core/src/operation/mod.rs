use crate::environment;
use crate::environment::Environment;
use crate::number::Number;

pub mod stack_manipulation;
pub mod basic_math;

pub trait Operation<N: Number, E: Environment<N>> {
    fn evaluate(&self, environment: &mut E) -> Result<(), Error>;
}

#[derive(Debug)]
pub enum Error {
    Environment(environment::Error),
}

impl From<environment::Error> for Error {
    fn from(value: environment::Error) -> Self {
        Error::Environment(value)
    }
}
