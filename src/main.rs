mod lexer;
use std::env;
use lexer::token::{Token, TokenType, };
use lexer::Lexer;

fn main() {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: bin [script]");
            std::process::exit(exitcode::NOINPUT)       
        }
    }
}
fn run_file(file_path: &str) {
    match std::fs::read_to_string(file_path){
        Ok(file) => run(file),
        Err(_) => println!("Error: Could not load file"),
    };
}
fn run(file: String) {
    let mut lexical: Lexer = Lexer::new(file);

    lexical.scan_tokens();
    lexical.print_tokens();
}
