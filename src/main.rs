use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use rlox::scanner;
use rlox::token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("Usage: rlox [script]");
        process::exit(exitcode::USAGE)
    } else if args.len() == 2 {
        run_file(&args[0])
    } else {
        run_prompt();
    }
}

fn run_file(file: &String) {
    let contents: String =
        fs::read_to_string(file).unwrap_or_else(|_| process::exit(exitcode::IOERR));
    run(&contents)
}

fn run(contents: &String) {
    let tokens: Vec<token::Token> = scanner::scan_tokens(contents.to_string());

    for token in tokens {
        println!("{token}")
    }
}

fn run_prompt() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        print!("> ");
	io::stdout().flush().unwrap();
        stdin
            .read_line(&mut buffer)
            .unwrap_or_else(|_| process::exit(exitcode::IOERR));
        if buffer.len() <= 1 {
            break
        }
        run(&buffer);
	buffer.clear()
    }
}
