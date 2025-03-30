use crate::environment::Environment;
use crate::number::Number;
use crate::operation::{Error, Operation};

pub struct Add {}

impl Add {
    pub fn new() -> Self {
        Self {}
    }
}

impl<N: Number, E: Environment<N>> Operation<N, E> for Add {
    fn evaluate(&self, environment: &mut E) -> Result<(), Error> {
        let b = environment.pop()?;
        let a = environment.pop()?;
        let c = a + b;
        environment.push(c)?;
        Ok(())
    }
}

pub struct Sub {}

impl Sub {
    pub fn new() -> Self {
        Self {}
    }
}

impl<N: Number, E: Environment<N>> Operation<N, E> for Sub {
    fn evaluate(&self, environment: &mut E) -> Result<(), Error> {
        let b = environment.pop()?;
        let a = environment.pop()?;
        let c = a - b;
        environment.push(c)?;
        Ok(())
    }
}

pub struct Mul {}

impl Mul {
    pub fn new() -> Self {
        Self {}
    }
}

impl<N: Number, E: Environment<N>> Operation<N, E> for Mul {
    fn evaluate(&self, environment: &mut E) -> Result<(), Error> {
        let b = environment.pop()?;
        let a = environment.pop()?;
        let c = a * b;
        environment.push(c)?;
        Ok(())
    }
}

pub struct Div {}

impl Div {
    pub fn new() -> Self {
        Self {}
    }
}

impl<N: Number, E: Environment<N>> Operation<N, E> for Div {
    fn evaluate(&self, environment: &mut E) -> Result<(), Error> {
        let b = environment.pop()?;
        let a = environment.pop()?;
        let c = a / b;
        environment.push(c)?;
        Ok(())
    }
}
