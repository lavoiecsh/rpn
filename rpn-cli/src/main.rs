use rpn_core::operation::{OpStack, OperationError, add, subtract, divide, remainder, multiply, rotate, square, copy};
use rpn_core::stack::Stack;
use rpn_std::stack::VecStack;
use std::io;
use std::io::Write;

type N = i32;
type S = VecStack<N>;

fn main() -> Result<(), io::Error> {
    let mut environment = CliEnvironment::default();
    let mut input = String::new();
    while !environment.exited {
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        for operation in input.split_whitespace().map(parse_input) {
            match operation {
                ParsedInput::Operation(o) => environment.evaluate(o),
                ParsedInput::Push(n) => environment.push(n),
                ParsedInput::Exit => {
                    environment.exited = true;
                }
                ParsedInput::Unknown(o) => println!("Unknown command: {o}"),
            }
        }
        input.clear();
        environment.print();
    }
    Ok(())
}

fn parse_input(input: &str) -> ParsedInput {
    match input {
        "+" | "add" => ParsedInput::Operation(add),
        "-" | "subtract" => ParsedInput::Operation(subtract),
        "*" | "multiply" => ParsedInput::Operation(multiply),
        "/" | "divide" => ParsedInput::Operation(divide),
        "%" | "remainder" => ParsedInput::Operation(remainder),
        "^2" | "square" => ParsedInput::Operation(square),
        "rotate" => ParsedInput::Operation(rotate),
        "copy" => ParsedInput::Operation(copy),
        "exit" => ParsedInput::Exit,
        _ => match input.parse() {
            Ok(n) => ParsedInput::Push(n),
            Err(..) => ParsedInput::Unknown(input.to_owned()),
        },
    }
}

enum ParsedInput {
    Operation(fn(OpStack<S>) -> Result<OpStack<S>, OperationError>),
    Unknown(String),
    Push(N),
    Exit,
}

#[derive(Default)]
struct CliEnvironment {
    stack: S,
    exited: bool,
}

impl CliEnvironment {
    fn print(&self) {
        self.stack
            .iter()
            .enumerate()
            .for_each(|(i, v)| println!("{i:2}: {v}"));
    }

    fn evaluate(&mut self, f: fn(OpStack<S>) -> Result<OpStack<S>, OperationError>) {
        match self.stack.evaluate(f) {
            Ok(new_stack) => self.stack = new_stack,
            Err(e) => println!("{e}"),
        }
    }

    fn push(&mut self, n: N) {
        match self.stack.push(n) {
            Ok(()) => {}
            Err(e) => println!("{e}"),
        }
    }
}
