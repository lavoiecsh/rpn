use crate::environment::Environment;
use crate::number::Number;
use crate::operation;
use crate::operation::Operation;

pub struct Push<N: Number> {
    number: N,
}

impl<N: Number> Push<N> {
    pub fn new(number: N) -> Self {
        Self { number }
    }
}

impl<N: Number, E: Environment<N>> Operation<N, E> for Push<N> {
    fn evaluate(&self, environment: &mut E) -> Result<(), operation::Error> {
        environment.push(self.number)?;
        Ok(())
    }
}

pub struct Pop {}

impl Pop {
    pub fn new() -> Self {
        Self {}
    }
}

impl<N: Number, E: Environment<N>> Operation<N, E> for Pop {
    fn evaluate(&self, environment: &mut E) -> Result<(), operation::Error> {
        environment.pop()?;
        Ok(())
    }
}

pub struct Rotate {}

impl Rotate {
    pub fn new() -> Self {
        Self {}
    }
}

impl<N: Number, E: Environment<N>> Operation<N, E> for Rotate {
    fn evaluate(&self, environment: &mut E) -> Result<(), operation::Error> {
        let b = environment.pop()?;
        let a = environment.pop()?;
        environment.push(b)?;
        environment.push(a)?;
        Ok(())
    }
}