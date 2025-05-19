use rpn_core::number::NumberError;
use rpn_core::operation::{
    Add, Divide, Multiply, OperationError, Pop, Push, Remainder, Rotate, Square, Subtract,
};
use rpn_core::stack::{Stack, StackError};
use rpn_std::stack::VecStack;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;

#[wasm_bindgen]
pub struct WasmStack {
    stack: VecStack<i32>,
    history: Vec<String>,
}

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl WasmStack {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            stack: VecStack::default(),
            history: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn stack(&self) -> Vec<String> {
        self.stack.iter()
            .cloned()
            .enumerate()
            .map(|(i,v)| format!("{i:3}: {v}"))
            .collect()
    }
    
    #[wasm_bindgen]
    pub fn history(&self) -> Vec<String> {
        self.history.clone()
    }

    #[wasm_bindgen]
    pub fn push(&mut self, value: i32) {
        match self.push_result(value) {
            Ok(new_stack) => {
                self.stack = new_stack;
                self.history.push(format!("{value}"));
            }
            Err(e) => {
                console::error_1(&e.0.into());
            }
        }
    }

    fn push_result(&self, value: i32) -> Result<VecStack<i32>, WasmError> {
        Ok(self.stack.evaluate(Push(value))?)
    }

    #[wasm_bindgen]
    pub fn evaluate(&mut self, input: char) {
        match self.evaluate_result(input) {
            Ok(new_stack) => {
                self.stack = new_stack;
                self.history.push(format!("{input}"));
            }
            Err(e) => {
                console::error_1(&e.0.into());
            }
        }
    }

    fn evaluate_result(&self, input: char) -> Result<VecStack<i32>, WasmError> {
        // todo autofocus input on start
        // todo handle <tab> for rotate
        // todo handle <enter> for push
        // todo don't add <space>/<enter> in history
        // todo change <tab> in history
        // todo add error message display
        // todo support dark mode
        // todo prettify UI
        // todo deploy to lavoiecsh.github.io
        // todo add GitHub Actions to build/test
        match input {
            '+' => Ok(self.stack.evaluate(Add)?),
            '-' => Ok(self.stack.evaluate(Subtract)?),
            '*' => Ok(self.stack.evaluate(Multiply)?),
            '/' => Ok(self.stack.evaluate(Divide)?),
            '%' => Ok(self.stack.evaluate(Remainder)?),
            's' => Ok(self.stack.evaluate(Square)?),
            'p' => Ok(self.stack.evaluate(Pop)?),
            '\t' => Ok(self.stack.evaluate(Rotate)?),
            ' ' => Ok(self.stack.clone()),
            _ => Err(WasmError(format!("invalid operation {input}"))),
        }
    }
}

struct WasmError(String);

impl From<OperationError> for WasmError {
    fn from(value: OperationError) -> Self {
        WasmError(match value {
            OperationError::Stack(StackError::Empty) => String::from("Stack is empty"),
            OperationError::Stack(StackError::SizeExceeded(n)) => {
                format!("Stack size exceeded, maximum = {n}")
            }
            OperationError::Number(NumberError::DivisionByZero) => {
                String::from("Invalid division by zero")
            }
            OperationError::Number(NumberError::Unchecked) => {
                String::from("Number underflowed or overflowed")
            }
        })
    }
}
