use rpn_core::operation::basic_math::{Add, Div, Mul, Sub};
use rpn_core::operation::stack_manipulation::{Pop, Push};
use rpn_core::operation::{Operation, OperationError};
use rpn_core::stack::large::LargeStack;
use rpn_core::stack::Stack;
use std::io::{Error, Write};

type N = i64;

fn main() {
    let mut stack = LargeStack::new();
    loop {
        match read() {
            Ok(operations) => {
                for operation in operations {
                    let result = evaluate(&operation, &mut stack);
                    if let Err(e) = result {
                        println!("Error evaluating command: {e:?}");
                        break;
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
    Operation(&'a str, OperationError<N>),
    UnknownOperation(&'a str),
}

impl<'a> From<Error> for ReplError<'a> {
    fn from(value: Error) -> Self {
        ReplError::IO(value)
    }
}

fn evaluate<'a>(operation: &'a str, stack: &mut impl Stack<N>) -> Result<(), ReplError<'a>> {
    let error_mapper = |o: OperationError<N>| ReplError::Operation(operation, o);
    match operation {
        "pop" => Pop.evaluate(stack).map_err(error_mapper),
        "+" => Add.evaluate(stack).map_err(error_mapper),
        "-" => Sub.evaluate(stack).map_err(error_mapper),
        "*" => Mul.evaluate(stack).map_err(error_mapper),
        "/" => Div.evaluate(stack).map_err(error_mapper),
        _ => if let Ok(n) = operation.parse::<N>() {
            Push::new(n).evaluate(stack).map_err(error_mapper)
        } else {
            Err(ReplError::UnknownOperation(operation))
        }
    }
}

fn print(stack: &impl Stack<N>) {
    stack
        .iter()
        .enumerate()
        .for_each(|(i,v)| println!("{:2}: {}", i, v));
}
