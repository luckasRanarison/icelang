use icelang::{cli::repl_mode, lexer::Lexer, parser::Parser, runtime::interpreter::Interpreter};
use std::{env, fs::read_to_string, process};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.len() {
        0 => repl_mode(),
        1 => run_file(&args[0]),
        _ => eprintln!("Invalid number of arguments"),
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
