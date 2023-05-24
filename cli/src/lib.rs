use std::{error::Error, path::PathBuf};

use interpreter::{
    builtin::{get_io_builtins, get_std_builtins},
    value::{Range, Value},
    Interpreter,
};
use lexer::Lexer;
use nu_ansi_term::{AnsiGenericString, Color};
use parser::Parser;
use reedline::{Reedline, Signal};

use self::{highlighter::IceHighlighter, prompt::IcePrompt, validator::IceValidator};

mod highlighter;
mod prompt;
mod validator;

pub fn print_errror<T: Error>(error_type: &str, error: T) {
    println!("{}: {}", Color::Red.paint(error_type), error)
}

pub fn color_value<'a>(value: &Value) -> AnsiGenericString<'a, str> {
    match value {
        Value::Number(value) => {
            let mut s = value.to_string();

            if s.ends_with(".0") {
                s.truncate(s.len() - 2);
            }

            Color::LightRed.paint(s)
        }
        Value::String(value) => Color::LightGreen.paint(format!("\"{value}\"")),
        Value::Boolean(value) => Color::Cyan.paint(format!("{:?}", value)),
        Value::Null => Color::DarkGray.paint("null"),
        Value::Array(items) => {
            let mut s = String::new();
            let mut iter = items.iter();
            if let Some(item) = iter.next() {
                s.push_str(&format!("{}", color_value(&item.borrow())));
                for item in iter {
                    s.push_str(&format!(", {}", color_value(&item.borrow())));
                }
            }
            Color::White.paint(format!("[{}]", s))
        }
        Value::Function(function) => {
            let name = match &function.declaration.token {
                Some(token) => &token.lexeme,
                None => "anonymous",
            };
            Color::LightBlue.paint(format!("[Function {}]", name))
        }
        Value::Builtin(builtin) => Color::LightBlue.paint(format!("[Function {}]", builtin.name)),
        Value::Object(object) => {
            let mut s = String::new();
            let mut iter = object.values.iter();
            if let Some((key, value)) = iter.next() {
                s.push_str(&format!("{}: {}", key, color_value(&value.borrow())));
                for (key, value) in iter {
                    s.push_str(&format!(", {}: {}", key, color_value(&value.borrow())));
                }
            }
            Color::White.paint(format!("{{ {} }}", s))
        }
        Value::Range(range) => {
            let (start, end) = match range {
                Range::NumberRange(value) => (
                    Value::Number(value.start as f64),
                    Value::Number(value.end as f64),
                ),
                Range::CharRange(value) => (
                    Value::String(value.start.to_string()),
                    Value::String(value.end.to_string()),
                ),
            };
            Color::White.paint(format!(
                "{}{}{}",
                color_value(&start),
                Color::LightBlue.paint(".."),
                color_value(&end)
            ))
        }
    }
}

pub fn repl_mode() {
    print!("Welcome to icelang REPL mode, MIT LICENSE (Press CTRL-C to exit)");

    let interpreter = Interpreter::new(PathBuf::new());
    interpreter.load_builtin(get_std_builtins());
    interpreter.load_builtin(get_io_builtins());
    let prompt = IcePrompt::new();
    let validator = Box::new(IceValidator::new());
    let highlighter = Box::new(IceHighlighter::new());
    let mut line_editor = Reedline::create()
        .with_highlighter(highlighter)
        .with_validator(validator);

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                let tokens = Lexer::new(&buffer).tokenize();
                let tokens = match tokens {
                    Ok(value) => value,
                    Err(err) => {
                        print_errror("Parsing error", err);
                        continue;
                    }
                };
                let nodes = Parser::new(&tokens).parse();
                let nodes = match nodes {
                    Ok(value) => value,
                    Err(err) => {
                        print_errror("Syntax error", err);
                        continue;
                    }
                };

                if nodes.is_empty() {
                    println!("{}", Color::DarkGray.paint("null"));
                }

                for node in nodes {
                    let value = interpreter.interpret(node);
                    match value {
                        Ok(value) => {
                            if let Some(value) = value {
                                println!("{}", color_value(&value));
                            }
                        }
                        Err(err) => {
                            print_errror("Runtime error", err);
                        }
                    };
                }
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("Bye!");
                break;
            }
            Err(error) => {
                eprintln!("{error}")
            }
        }
    }
}
