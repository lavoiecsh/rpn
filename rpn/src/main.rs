use rpn_core::environment::large_stack::LargeStackEnvironment;
use rpn_core::environment::Environment;
use rpn_core::operation::basic_math::{Add, Div, Mul, Sub};
use rpn_core::operation::stack_manipulation::{Pop, Push, Rotate};
use rpn_core::operation::Operation;
use std::io::Write;

type N = i64;
type E = LargeStackEnvironment<N>;
type O = dyn Operation<N, E>;

fn main() {
    let mut environment = E::new();
    loop {
        let read_result = read();
        if let Ok(operations) = read_result {
            for operation in operations {
                let result = operation.evaluate(&mut environment);
                if let Err(e) = result {
                    println!("Error evaluating command: {e:?}");
                    break;
                }
            }
        } else {
            println!("Error parsing input: {}", read_result.err().unwrap());
        }
        print(&environment);
    }
}

fn read() -> Result<Vec<Box<O>>, std::io::Error> {
    print!("> ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if input.contains("exit") {
        std::process::exit(0);
    }
    let operations = input.split_whitespace()
        .filter_map(parse)
        .collect::<Vec<_>>();
    Ok(operations)
}

fn parse(token: &str) -> Option<Box<O>> {
    match token {
        "pop" => Some(Box::new(Pop::new())),
        "rotate" => Some(Box::new(Rotate::new())),
        "+" => Some(Box::new(Add::new())),
        "-" => Some(Box::new(Sub::new())),
        "*" => Some(Box::new(Mul::new())),
        "/" => Some(Box::new(Div::new())),
        _ => if let Ok(n) = token.parse::<i64>() {
            Some(Box::new(Push::new(n)))
        } else {
            None
        }
    }
}

fn print(environment: &E) {
    environment.stack()
        .enumerate()
        // .rev()
        .for_each(|(i,v)| println!("{:2}: {}", i, v));
}
