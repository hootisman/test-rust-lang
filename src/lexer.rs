pub mod token;
use std::{ iter::Peekable, vec::IntoIter};

use self::token::{Token, TokenType, LiteralType};

pub struct Lexer<'a> {
    pub src: Peekable<IntoIter<u8>>,
    pub tokens: Vec<Token<'a>>,
    pub line: u16,
}

impl<'a> Lexer<'a>{
    pub fn new<'b>(src: String) -> Lexer<'b>{
        let tokens: Vec<Token<'b>> = Vec::new();
        Lexer {
            src: src.into_bytes().into_iter().peekable(),
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
    fn scan_token(&mut self, c: char){
        println!("{}",c );
        match c {
            '-' => self.add_token(TokenType::MINUS, "-"),
            '+' => self.add_token(TokenType::PLUS, "+"),
            '.' => self.add_token(TokenType::DOT, "."),
            '*' => self.add_token(TokenType::STAR, "*"),
            ';' => self.add_token(TokenType::SEMICOLON, ";"),
            '(' => self.add_token(TokenType::LEFTPAREN, "("),
            ')' => self.add_token(TokenType::RIGHTPAREN, ")"),
            id @ 'a'..='z' | id @ 'A'..='Z' => self.add_token(TokenType::IDENTIFER(id), ""),
            num @ '0'..='9' => {
                self.add_token(TokenType::NUMBER(num.to_digit(10).unwrap()), ")");
            },
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => (),
            _ => self.syntax_error("Found unknown character"),
        }
    }
    pub fn scan_tokens(&mut self){
        loop {
            //keep iterating through source code until at EOF
            match self.src.next() {
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
