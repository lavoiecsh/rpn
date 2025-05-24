use rpn_core::operation::{
    Add, Divide, Multiply, Pop, Push, Remainder, Rotate, Square, Subtract,
};
use rpn_core::stack::Stack;
use rpn_std::stack::VecStack;
use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;

#[wasm_bindgen]
pub struct WasmEnvironment {
    stack: VecStack<i32>,
    history: Vec<String>,
}

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl WasmEnvironment {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            stack: VecStack::default(),
            history: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn stack(&self) -> Vec<String> {
        self.stack
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, v)| format!("{i:3}: {v}"))
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
                console::error_1(&format!("{e}").into());
            }
        }
    }

    fn push_result(&self, value: i32) -> Result<VecStack<i32>, Box<dyn std::error::Error>> {
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
                console::error_1(&format!("{e}").into());
            }
        }
    }

    fn evaluate_result(&self, input: char) -> Result<VecStack<i32>, Box<dyn std::error::Error>> {
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
            _ => Err(Box::new(WasmError(format!("Invalid operation: {input}")))),
        }
    }
}

#[derive(Debug)]
struct WasmError(String);

impl Display for WasmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for WasmError {}
