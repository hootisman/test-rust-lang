mod lexer;
use std::env;
use lexer::token::{Token, TokenType, };
use lexer::Lexer;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: bin [script]");
        std::process::exit(exitcode::NOINPUT)       
    }else if args.len() == 2 {
        //runfile(file)
        // run_file(&(args[1])[..]);
        run_file(&args[1]);
    }else{
        //runprompt
        
    }

    // let test: Token = Token{ ttype: TokenType::STAR, lexeme: "2322222323", literal: LiteralType::number(2322222232), line: 2 };
    //
    // println!("{}",test.to_string());
    

}
fn run_file(file_path: &str) {
    // let file = match std::fs::read_to_string(file_path){
    //                 Ok(t) => t,
    //                 Err(_) => String::from("Error, could not load file"),
    //             };
    match std::fs::read_to_string(file_path){
        Ok(file) => run(file),
        Err(_) => println!("Error: Could not load file"),
    };
}
fn run(file: String) {
    let mut leex: Lexer = Lexer::new(file);
        // let test1 = Token{
        //     ttype: TokenType::MINUS,
        //     lexeme: "-",
        //     literal: LiteralType::char('-'),
        //     line: 43,
        // };
        //
        // let test2 = Token{
        //     ttype: TokenType::NUMBER,
        //     lexeme: "23123",
        //     literal: LiteralType::number(23123),
        //     line: 43,
        // };
        // leex.tokens.push(test1);
        // leex.tokens.push(test2);
        //
        // for token in leex.tokens.iter() {
        //     println!("{}",token.to_string());
        // }
    leex.scan_tokens();
    leex.print_tokens();
}
