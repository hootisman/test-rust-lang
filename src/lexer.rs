pub mod token;
use std::{ iter::Peekable, vec::IntoIter};

use self::token::{Token, TokenType };

pub struct Lexer<'a> {
    pub source: Peekable<IntoIter<u8>>,
    pub tokens: Vec<Token<'a>>,
    pub line: u16,
}

impl<'a> Lexer<'a>{
    /// Creates a new Lexical Analyzer with the 'peekable' source code
    pub fn new<'b>(src: String) -> Lexer<'b>{
        let tokens: Vec<Token<'b>> = Vec::new();
        Lexer {
            source: src.into_bytes().into_iter().peekable(),
            tokens,
            line: 1,
        }
    }
    fn add_token(&mut self, ttype: TokenType, lexeme: &'a str){
        self.tokens.push( Token{
                            ttype,
                            lexeme,
                            line: self.line,
                        });
    }
    ///adds a token with an i32 number created by checking for more number chars in the source code
    fn add_number_token(&mut self, num: char){
        let mut literal = String::new();
        literal.push(num); //first char

        loop {
            match self.source.peek(){
                Some(c) if (*c as char).is_digit(10) => {
                    //next char is a number, add the next char to the literal
                    literal.push(self.source.next().unwrap() as char);
                },
                Some(_) | None => break,
            }
        }
        let literal_i32: i32 = literal.parse::<i32>().unwrap();  //the literal as i32
        self.add_token(TokenType::NUMBER(literal_i32), "");
    }
    fn scan_token(&mut self, c: char){
        match c {
            '+' => self.add_token(TokenType::PLUS, "+"),
            '.' => self.add_token(TokenType::DOT, "."),
            '*' => self.add_token(TokenType::STAR, "*"),
            ';' => self.add_token(TokenType::SEMICOLON, ";"),
            '=' => self.add_token(TokenType::EQUAL, "="),
            '(' => self.add_token(TokenType::LEFTPAREN, "("),
            ')' => self.add_token(TokenType::RIGHTPAREN, ")"),
            '-' => self.add_token(TokenType::MINUS, "-"),
            id @ 'a'..='z' | id @ 'A'..='Z' => self.add_token(TokenType::IDENTIFER(id), ""),
            num @ '0'..='9' => self.add_number_token(num),
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => (),
            _ => self.syntax_error("Found unknown character"),
        }
    }
    pub fn scan_tokens(&mut self){
        loop {
            //keep iterating through source code until at end
            match self.source.next() {
                Some(c) => self.scan_token(c as char),
                None => break,
            }
        }

        self.add_token(TokenType::EOF, "");      //EOF token
    }
    pub fn print_tokens(&self){
        for token in self.tokens.iter() {
            println!("{}",token.to_string());
        }
    }
    fn syntax_error(&self, message: &str){
        eprintln!("[line: {}] ERROR: {}",self.line,message);
        std::process::exit(exitcode::USAGE)
    }
}
