use rpn_core::operation::{Add, Divide, Multiply, Operation, OperationError, Pop, Push, Subtract};
use rpn_core::stack::{LargeStack, Stack};
use std::io::{Error, Write};

type N = i32;
type S = LargeStack<N>;

fn main() {
    let mut stack = LargeStack::default();
    loop {
        match read() {
            Ok(operations) => {
                for operation in operations {
                    match evaluate(&operation, &stack) {
                        Ok(new_stack) => { stack = new_stack; },
                        Err(e) => { println!("Error evaluating command: {e:?}") },
                    }
                }
            }
            Err(ReplError::Exit) => {
                println!("Exiting.");
                std::process::exit(0);
            }
            Err(ReplError::IO(error)) => {
                println!("Error parsing input: {error}");
            }
            Err(ReplError::UnknownOperation(op)) => {
                println!("Unknown operation: {op}");
            }
            Err(ReplError::Operation(op, error)) => {
                println!("Error evaluating {op}: {error:?}");
            }
        }
        print(&stack);
    }
}

fn read<'a>() -> Result<Vec<String>, ReplError<'a>> {
    print!("> ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if input.contains("exit") {
        return Err(ReplError::Exit);
    }
    Ok(input.split_whitespace()
        .map(String::from)
        .collect::<Vec<_>>())
}

#[derive(Debug)]
enum ReplError<'a> {
    Exit,
    IO(Error),
    Operation(&'a str, OperationError),
    UnknownOperation(&'a str),
}

impl<'a> From<Error> for ReplError<'a> {
    fn from(value: Error) -> Self {
        ReplError::IO(value)
    }
}

fn evaluate<'a>(operation: &'a str, stack: &S) -> Result<S, ReplError<'a>> {
    let error_mapper = |o: OperationError| ReplError::Operation(operation, o);
    match operation {
        "pop" => Pop.evaluate(stack).map_err(error_mapper),
        "+" => Add.evaluate(stack).map_err(error_mapper),
        "-" => Subtract.evaluate(stack).map_err(error_mapper),
        "*" => Multiply.evaluate(stack).map_err(error_mapper),
        "/" => Divide.evaluate(stack).map_err(error_mapper),
        _ => if let Ok(n) = operation.parse::<N>() {
            Push(n).evaluate(stack).map_err(error_mapper)
        } else {
            Err(ReplError::UnknownOperation(operation))
        }
    }
}

fn print(stack: &impl Stack<N>) {
    stack
        .iter()
        .enumerate()
        .for_each(|(i,v)| println!("{i:2}: {v}"));
}
