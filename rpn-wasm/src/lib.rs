use rpn_core::number::NumberError;
use rpn_core::operation::{
    Add, Divide, Multiply, OperationError, Pop, Push, Remainder, Rotate, Square, Subtract,
};
use rpn_core::stack::{Stack, StackError};
use rpn_std::stack::VecStack;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;

#[wasm_bindgen]
pub struct WasmStack(VecStack<i32>);

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl WasmStack {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(VecStack::default())
    }

    #[wasm_bindgen]
    pub fn stack(&self) -> Vec<i32> {
        self.0.iter().cloned().collect()
    }

    #[wasm_bindgen]
    pub fn push(&mut self, value: i32) {
        if let Err(e) = self.push_result(value) {
            console::error_1(&e.0.into());
        }
    }

    fn push_result(&mut self, value: i32) -> Result<(), WasmError> {
        self.0 = self.0.evaluate(Push(value))?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn evaluate(&mut self, input: char) {
        match self.evaluate_result(input) {
            Ok(new_stack) => {
                self.0 = new_stack;
            }
            Err(e) => {
                console::error_1(&e.0.into());
            }
        }
    }
    
    fn evaluate_result(&self, input: char) -> Result<VecStack<i32>, WasmError> {
        match input {
            '+' => Ok(self.0.evaluate(Add)?),
            '-' => Ok(self.0.evaluate(Subtract)?),
            '*' => Ok(self.0.evaluate(Multiply)?),
            '/' => Ok(self.0.evaluate(Divide)?),
            '%' => Ok(self.0.evaluate(Remainder)?),
            's' => Ok(self.0.evaluate(Square)?),
            'p' => Ok(self.0.evaluate(Pop)?),
            '\t' => Ok(self.0.evaluate(Rotate)?),
            ' ' => Ok(self.0.clone()),
            _ => {
                Err(WasmError(format!("invalid operation {input}")))
            },
        }
    }
}

struct WasmError(String);

impl From<OperationError> for WasmError {
    fn from(value: OperationError) -> Self {
        match value {
            OperationError::Stack(StackError::Empty) => WasmError(String::from("Stack is empty")),
            OperationError::Stack(StackError::SizeExceeded(n)) => {
                WasmError(format!("Stack size exceeded, maximum = {n}"))
            }
            OperationError::Number(NumberError::DivisionByZero) => {
                WasmError(String::from("Invalid division by zero"))
            }
            OperationError::Number(NumberError::Unchecked) => {
                WasmError(String::from("Number underflowed or overflowed"))
            }
        }
    }
}
