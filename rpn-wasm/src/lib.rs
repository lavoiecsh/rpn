use rpn_core::operation::{
    add, copy, divide, multiply, remainder, rotate, subtract, OpStack, OperationError,
};
use rpn_core::stack::Stack;
use rpn_std::stack::VecStack;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{console, window, Element, HtmlInputElement, InputEvent};

type S = VecStack<i32>;

#[wasm_bindgen]
pub struct WasmEnvironment {
    stack: S,
    history: Vec<String>,

    input_element: HtmlInputElement,
    stack_element: Element,
    history_element: Element,
}

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl WasmEnvironment {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let window = window().expect("no window element found");
        let document = window.document().expect("no document element found");
        Self {
            stack: VecStack::default(),
            history: Vec::new(),
            input_element: document
                .get_element_by_id("input")
                .expect("no input element found")
                .dyn_into::<HtmlInputElement>()
                .expect("input element is not an input"),
            stack_element: document
                .get_element_by_id("stack")
                .expect("no stack element found"),
            history_element: document
                .get_element_by_id("history")
                .expect("no history element found"),
        }
    }

    fn clear_print(&self) {
        self.input_element.set_value("");
        self.print_stack();
        self.print_history();
    }

    fn print_stack(&self) {
        self.stack_element.set_inner_html(
            &self
                .stack
                .iter()
                .enumerate()
                .map(|(index, item)| format!("<tr><td>{index}</td><td>{item}</td></tr>"))
                .collect::<String>(),
        );
    }

    fn print_history(&self) {
        self.history_element.set_inner_html(
            &self
                .history
                .iter()
                .map(|item| format!("<li>{item}</li>"))
                .collect::<String>(),
        );
    }

    #[wasm_bindgen]
    pub fn evaluate(&mut self, event: InputEvent) {
        let action = match event.data().and_then(|s| s.chars().next()) {
            Some(' ') => WasmInputAction::Push,
            Some('+') => WasmInputAction::Operation(add, "+"),
            Some('-') => WasmInputAction::Operation(subtract, "-"),
            Some('*') => WasmInputAction::Operation(multiply, "*"),
            Some('/') => WasmInputAction::Operation(divide, "/"),
            Some('%') => WasmInputAction::Operation(remainder, "%"),
            Some('r') => WasmInputAction::Operation(rotate, "rotate"),
            Some('c') => WasmInputAction::Operation(copy, "copy"),
            Some('0') | Some('1') | Some('2') | Some('3') | Some('4') | Some('5') | Some('6')
            | Some('7') | Some('8') | Some('9') => WasmInputAction::Ignore,
            Some(c) => WasmInputAction::Unknown(c),
            None => WasmInputAction::Ignore,
        };
        match action {
            WasmInputAction::Push => {
                self.push_from_input();
                self.clear_print();
            }
            WasmInputAction::Ignore => {
            }
            WasmInputAction::Unknown(c) => {
                console::error_1(&format!("unknown operation {c}").into());
                self.clear_print();
            }
            WasmInputAction::Operation(o, name) => {
                self.push_from_input();
                match self.stack.evaluate(o) {
                    Ok(new_stack) => {
                        self.stack = new_stack;
                        self.history.push(name.into());
                    }
                    Err(e) => console::error_1(&format!("failed operation {e}").into()),
                }
                self.clear_print();
            }
        }
    }

    fn push_from_input(&mut self) {
        let value = self.input_element.value();
        let cleaned = value.trim_end_matches(|c| !"0123456789".contains(c));
        match cleaned.parse() {
            Ok(number) => {
                self.stack.push(number).expect("failed pushing number");
                self.history.push(cleaned.into());
            }
            Err(e) => {
                console::error_1(&format!("failed to parse number: {e}").into());
            }
        }
    }
}

enum WasmInputAction<'a> {
    Operation(fn(OpStack<S>) -> Result<OpStack<S>, OperationError>, &'a str),
    Push,
    Ignore,
    Unknown(char),
}
