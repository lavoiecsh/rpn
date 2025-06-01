use crate::number::NumberError;
use crate::stack::{Stack, StackError};
use core::error::Error;
use core::fmt::{Display, Formatter};

mod add;
mod copy;
mod divide;
mod multiply;
mod remainder;
mod rotate;
mod square;
mod subtract;

pub use add::add;
pub use copy::copy;
pub use divide::divide;
pub use multiply::multiply;
pub use remainder::remainder;
pub use rotate::rotate;
pub use square::square;
pub use subtract::subtract;

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

#[derive(Debug)]
pub struct OperationStack<S: Stack, T> {
    stack: S,
    state: T,
}

pub type OpStack<S> = OperationStack<S, NoItems>;

impl<S: Stack, T> OperationStack<S, T> {
    pub(crate) fn stack(self) -> S {
        self.stack
    }
}

#[derive(Debug)]
pub struct NoItems;
#[derive(Debug)]
pub struct OneItem<T>(T);
#[derive(Debug)]
pub struct TwoItems<T>(T, T);

impl<S: Stack> OperationStack<S, NoItems> {
    pub fn new(stack: S) -> Self {
        Self {
            stack,
            state: NoItems,
        }
    }

    pub fn pop(mut self) -> Result<OperationStack<S, OneItem<S::Item>>, OperationError> {
        let item = self.stack.pop()?;
        Ok(OperationStack {
            stack: self.stack,
            state: OneItem(item),
        })
    }
}

impl<S: Stack> OperationStack<S, OneItem<S::Item>> {
    pub fn pop(mut self) -> Result<OperationStack<S, TwoItems<S::Item>>, OperationError> {
        let item = self.stack.pop()?;
        Ok(OperationStack {
            stack: self.stack,
            state: TwoItems(self.state.0, item),
        })
    }

    pub fn push(mut self) -> Result<OperationStack<S, NoItems>, OperationError> {
        self.stack.push(self.state.0)?;
        Ok(OperationStack {
            stack: self.stack,
            state: NoItems,
        })
    }
}

impl<S: Stack> OperationStack<S, OneItem<S::Item>>
where
    S::Item: Copy,
{
    pub fn copy(self) -> Result<OperationStack<S, TwoItems<S::Item>>, OperationError> {
        Ok(OperationStack {
            stack: self.stack,
            state: TwoItems(self.state.0, self.state.0),
        })
    }
}

impl<S: Stack> OperationStack<S, TwoItems<S::Item>> {
    pub fn push(mut self) -> Result<OperationStack<S, OneItem<S::Item>>, OperationError> {
        self.stack.push(self.state.1)?;
        Ok(OperationStack {
            stack: self.stack,
            state: OneItem(self.state.0),
        })
    }

    pub fn combine(
        self,
        f: impl FnOnce(S::Item, S::Item) -> Result<S::Item, OperationError>,
    ) -> Result<OperationStack<S, OneItem<S::Item>>, OperationError> {
        let item = f(self.state.1, self.state.0)?;
        Ok(OperationStack {
            stack: self.stack,
            state: OneItem(item),
        })
    }

    pub fn rotate(self) -> Result<OperationStack<S, TwoItems<S::Item>>, OperationError> {
        Ok(OperationStack {
            stack: self.stack,
            state: TwoItems(self.state.1, self.state.0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::SmallStack;
    use core::assert_matches::assert_matches;

    impl NoItems {
        fn inspect(self) {}
    }

    impl<T> OneItem<T> {
        fn inspect(self) -> T {
            self.0
        }
    }

    impl<T> TwoItems<T> {
        fn inspect(self) -> (T, T) {
            (self.0, self.1)
        }
    }

    #[test]
    fn no_item_stack_can_pop_item() {
        let no_items = OperationStack {
            stack: SmallStack::one_element(1),
            state: NoItems,
        };
        let one_item = no_items.pop().unwrap();
        assert_matches!(one_item.state.inspect(), 1);
        assert_matches!(one_item.stack.inspect(), (None, None));
    }

    #[test]
    fn one_item_stack_can_pop_item() {
        let one_item = OperationStack {
            stack: SmallStack::one_element(1),
            state: OneItem(2),
        };
        let two_items = one_item.pop().unwrap();
        assert_matches!(two_items.state.inspect(), (2, 1));
        assert_matches!(two_items.stack.inspect(), (None, None));
    }

    #[test]
    fn one_item_stack_can_push_item() {
        let one_item = OperationStack {
            stack: SmallStack::empty(),
            state: OneItem(1),
        };
        let no_items = one_item.push().unwrap();
        assert_matches!(no_items.state.inspect(), ());
        assert_matches!(no_items.stack.inspect(), (Some(1), None));
    }

    #[test]
    fn one_item_stack_can_copy_item() {
        let one_item = OperationStack {
            stack: SmallStack::empty(),
            state: OneItem(1),
        };
        let two_items = one_item.copy().unwrap();
        assert_matches!(two_items.state.inspect(), (1, 1));
        assert_matches!(two_items.stack.inspect(), (None, None));
    }

    #[test]
    fn two_item_stack_can_push_item() {
        let two_items = OperationStack {
            stack: SmallStack::empty(),
            state: TwoItems(2, 1),
        };
        let one_item = two_items.push().unwrap();
        assert_matches!(one_item.state.inspect(), 2);
        assert_matches!(one_item.stack.inspect(), (Some(1), None));
    }

    #[test]
    fn two_item_stack_can_combine() {
        let two_items = OperationStack {
            stack: SmallStack::empty(),
            state: TwoItems(2, 1),
        };
        let one_item = two_items.combine(|a, b| Ok(a + b)).unwrap();
        assert_matches!(one_item.state.inspect(), 3);
        assert_matches!(one_item.stack.inspect(), (None, None));
    }

    #[test]
    fn pushing_on_full_stack_errors() {
        let one_item = OperationStack {
            stack: SmallStack::two_elements(1, 2),
            state: OneItem(3),
        };
        let error = one_item.push();
        assert_matches!(
            error,
            Err(OperationError::Stack(StackError::SizeExceeded(2)))
        );
    }

    #[test]
    fn popping_on_empty_stack_errors() {
        let no_item = OperationStack {
            stack: SmallStack::<i32>::empty(),
            state: NoItems,
        };
        let error = no_item.pop();
        assert_matches!(error, Err(OperationError::Stack(StackError::Empty)));
    }
}
