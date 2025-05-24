use std::{error, io};
use rpn_core::number::Number;
use rpn_core::operation::{
    Add, Copy, Divide, Multiply, Pop, Push, Remainder, Rotate, Subtract,
};
use rpn_core::stack::Stack;
use rpn_std::stack::VecStack;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::str::FromStr;

fn main() -> Result<(), io::Error> {
    let mut environment = CliEnvironment::<i32>::default();
    let mut input = String::new();
    while !environment.exited {
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        for operation in input.split_whitespace() {
            environment.evaluate(operation);
        }
        input.clear();
        environment.print();
    }
    Ok(())
}

struct CliEnvironment<N: Number> {
    stack: VecStack<N>,
    exited: bool,
}

impl<N: Number> Default for CliEnvironment<N> {
    fn default() -> Self {
        Self {
            stack: VecStack::default(),
            exited: false,
        }
    }
}

impl<N: Number + Display + FromStr> CliEnvironment<N> {
    fn print(&self) {
        self.stack
            .iter()
            .enumerate()
            .for_each(|(i, v)| println!("{i:2}: {v}"));
    }
    
    fn evaluate(&mut self, input: &str) {
        match input {
            "exit" => self.exited = true,
            _ => match self.evaluate_result(input) {
                Ok(new_stack) => {
                    self.stack = new_stack;
                }
                Err(e) => {
                    println!("{e}");
                }
            }
        }
    }

    fn evaluate_result(&self, input: &str) -> Result<VecStack<N>, Box<dyn error::Error>> {
        match input {
            "+" => Ok(self.stack.evaluate(Add)?),
            "-" => Ok(self.stack.evaluate(Subtract)?),
            "*" => Ok(self.stack.evaluate(Multiply)?),
            "/" => Ok(self.stack.evaluate(Divide)?),
            "%" => Ok(self.stack.evaluate(Remainder)?),
            "pop" => Ok(self.stack.evaluate(Pop)?),
            "rot" => Ok(self.stack.evaluate(Rotate)?),
            "copy" => Ok(self.stack.evaluate(Copy)?),
            _ => {
                if let Ok(n) = input.parse::<N>() {
                    Ok(self.stack.evaluate(Push(n))?)
                } else {
                    Err(Box::new(CliError(format!("Invalid operation: {input}"))))
                }
            }
        }
    }
}

#[derive(Debug)]
struct CliError(String);

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for CliError {}