use std::{env, fs::read_to_string, io, process};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.len() {
        0 => repl_mode(),
        1 => run_file(&args[0]),
        _ => eprintln!("Invalid number of arguments"),
    }
}

fn repl_mode() {
    loop {
        print!("> ");
        io::Write::flush(&mut io::stdout()).expect("Error: output error");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error: failed to read line");
        input = input.trim().to_string();

        if input == "exit()" {
            println!("Bye!");
            process::exit(0);
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
