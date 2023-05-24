mod builtins;

use std::path::PathBuf;

use builtins::get_io_builtins;
use interpreter::{builtin::get_std_builtins, Interpreter};
use lexer::Lexer;
use parser::Parser;
use wasm_bindgen::prelude::*;
use web_sys::window;

pub fn print_to_output(text: &str) {
    let document = window().unwrap().document().unwrap();
    let output = document.get_element_by_id("output").unwrap();
    let div = document.create_element("div").unwrap();
    div.set_text_content(Some(text));
    output.append_child(&div).unwrap();
}

#[wasm_bindgen]
pub fn interprete(source: &str) -> String {
    let tokens = match Lexer::new(source).tokenize() {
        Ok(value) => value,
        Err(error) => return format!("Parsing error: {}", error),
    };
    let ast = match Parser::new(&tokens).parse() {
        Ok(value) => value,
        Err(error) => return format!("Syntax error: {}", error),
    };
    let interpreter = Interpreter::new(PathBuf::new());
    interpreter.load_builtin(get_std_builtins());
    interpreter.load_builtin(get_io_builtins());

    for node in ast {
        if let Some(error) = interpreter.interpret(node).err() {
            return format!("Runtime error: {}", error);
        };
    }

    String::new()
}
