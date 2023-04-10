use std::{env, fs::read_to_string, io, process, vec};

use icelang::{
    parser::parser::Parser,
    runtime::{interpreter::Interpreter, value::Value},
    tokenizer::{
        lexer::Lexer,
        tokens::{Token, TokenType},
        utils::Position,
    },
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
    let mut interpreter = Interpreter::new();
    println!("Welcome to icelang REPL mode!");

    loop {
        print!("> ");
        io::Write::flush(&mut io::stdout()).expect("Internal error: output error");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Internal error: failed to read line");
        input = input.trim().to_string();

        if input == "exit()" {
            println!("Bye!");
            process::exit(0);
        }

        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize().unwrap_or_else(|err| {
            println!("Parsing error: {}", err);
            vec![Token::new(
                TokenType::Eof,
                String::new(),
                Position::new(1, 0, 0),
            )]
        });
        let mut parser = Parser::new(&tokens);
        let nodes = parser.parse().unwrap_or_else(|err| {
            println!("Parsing error: {}", err);
            vec![]
        });

        if nodes.is_empty() {
            println!("null");
            continue;
        }

        for node in nodes {
            let value = interpreter.evaluate_statement(node).unwrap_or_else(|err| {
                println!("{}", err);
                Value::Null
            });

            println!("{}", value.stringify());
        }
    }
}

fn run_file(file_path: &String) {
    let contents = read_to_string(file_path).unwrap_or_else(|err| {
        let message = err.to_string();
        eprintln!("Error: {}", message);
        process::exit(1);
    });

    println!("{}", contents);
}
