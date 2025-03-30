use rpn_core::environment::Environment;
use rpn_core::environment::large_stack::LargeStackEnvironment;
use rpn_core::operation;
use rpn_core::operation::basic_math::{Add, Div, Mul, Sub};
use rpn_core::operation::Operation;
use rpn_core::operation::stack_manipulation::Push;

#[test]
fn acceptance_test() -> Result<(), operation::Error> {
    let mut env = LargeStackEnvironment::<i64>::new();
    Push::<i64>::new(21).evaluate(&mut env)?;
    Push::<i64>::new(34).evaluate(&mut env)?;
    Add::new().evaluate(&mut env)?;
    assert_eq!(env.stack_size(), 1);
    assert_eq!(env.stack().next().unwrap(), &55);

    Push::<i64>::new(10).evaluate(&mut env)?;
    Sub::new().evaluate(&mut env)?;
    assert_eq!(env.stack_size(), 1);
    assert_eq!(env.stack().next().unwrap(), &45);

    Push::<i64>::new(2).evaluate(&mut env)?;
    Mul::new().evaluate(&mut env)?;
    assert_eq!(env.stack_size(), 1);
    assert_eq!(env.stack().next().unwrap(), &90);

    Push::<i64>::new(5).evaluate(&mut env)?;
    Div::new().evaluate(&mut env)?;
    assert_eq!(env.stack_size(), 1);
    assert_eq!(env.stack().next().unwrap(), &18);
    Ok(())
}
