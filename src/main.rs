use cli::{print_errror, repl_mode};
use interpreter::{
    builtin::{get_io_builtins, get_std_builtins},
    Interpreter,
};
use lexer::Lexer;
use parser::Parser;
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
        print_errror("Internal error", err);
        process::exit(1);
    });

    let interpreter = Interpreter::new();
    interpreter.load_builtin(get_std_builtins());
    interpreter.load_builtin(get_io_builtins());

    let tokens = Lexer::new(&contents).tokenize().unwrap_or_else(|err| {
        print_errror("Parsing error", err);
        process::exit(1)
    });
    let nodes = Parser::new(&tokens).parse().unwrap_or_else(|err| {
        print_errror("Syntax error", err);
        process::exit(1)
    });

    for node in nodes {
        interpreter.interpret(node).unwrap_or_else(|err| {
            print_errror("Runtime error", err);
            process::exit(1)
        });
    }
}
