use std::error::Error;

use nu_ansi_term::Color;
use reedline::{Reedline, Signal};

use crate::{
    cli::{highlighter::IceHighlighter, prompt::IcePrompt, validator::IceValidator},
    lexer::Lexer,
    parser::Parser,
    runtime::interpreter::Interpreter,
};

mod highlighter;
mod prompt;
mod validator;

fn print_errror<T: Error>(error_type: &str, error: T) {
    println!("{}: {}", Color::Red.paint(error_type), error)
}

pub fn repl_mode() {
    print!("Welcome to icelang REPL mode, MIT LICENSE (Press CTRL-C to exit)");

    let interpreter = Interpreter::new();
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
                                println!("{}", value.paint());
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
