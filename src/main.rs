use std::{env, fs::read_to_string, io, process};

use icelang::{
    lexer::Lexer,
    parser::{error::ParsingError, Parser},
    runtime::{error::RuntimeError, interpreter::Interpreter},
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.len() {
        0 => repl_mode(),
        1 => run_file(&args[0]),
        _ => eprintln!("Invalid number of arguments"),
    }
}

fn repl_mode() {
    println!("Welcome to icelang REPL mode!");
    let interpreter = Interpreter::new();
    let mut input_queue = String::new();

    loop {
        match input_queue.is_empty() {
            true => print!("> "),
            false => {
                input_queue.push('\n');
                print!(".. ")
            }
        }

        io::Write::flush(&mut io::stdout()).expect("Internal error: output error");
        let mut current_input = String::new();
        io::stdin()
            .read_line(&mut current_input)
            .expect("Internal error: failed to read line");
        current_input = current_input.trim().to_string();

        if current_input == "exit()" {
            println!("Bye!");
            process::exit(0);
        }

        input_queue.push_str(&current_input);

        let mut lexer = Lexer::new(&input_queue);
        let tokens = lexer.tokenize();
        let tokens = match tokens {
            Ok(value) => value,
            Err(err) => {
                input_queue.clear();
                println!("Parsing error: {}", err);
                continue;
            }
        };
        let mut parser = Parser::new(&tokens);
        let nodes = parser.parse();
        let nodes = match nodes {
            Ok(value) => {
                input_queue.clear();
                value
            }
            Err(err) => match err {
                ParsingError::MissingParenthesis(_)
                | ParsingError::MissingClosingBrace(_)
                | ParsingError::ExpectedComma(_)
                | ParsingError::UnexpedtedEndOfInput(_) => {
                    continue;
                }
                _ => {
                    input_queue.clear();
                    println!("Syntax error: {}", err);
                    continue;
                }
            },
        };

        if nodes.is_empty() {
            println!("null");
        }

        for node in nodes {
            let value = interpreter.interpret(node);
            match value {
                Ok(value) => {
                    if let Some(value) = value {
                        println!("{}", value);
                    }
                }
                Err(err) => {
                    if let RuntimeError::Export(value) = err {
                        println!("{}", value);
                    } else {
                        println!("Runtime error: {}", err)
                    }
                }
            };
        }
    }
}

fn run_file(file_path: &String) {
    let contents = read_to_string(file_path).unwrap_or_else(|err| {
        let message = err.to_string();
        eprintln!("Error: {}", message);
        process::exit(1);
    });

    let interpreter = Interpreter::new();
    let mut lexer = Lexer::new(&contents);
    let tokens = lexer.tokenize().unwrap_or_else(|err| {
        eprintln!("Parsing error: {err}");
        process::exit(1)
    });
    let mut parser = Parser::new(&tokens);
    let nodes = parser.parse().unwrap_or_else(|err| {
        eprintln!("Syntax error: {err}");
        process::exit(1)
    });

    for node in nodes {
        interpreter.interpret(node).unwrap_or_else(|err| {
            eprintln!("Runtime error: {err}");
            process::exit(1)
        });
    }
}
