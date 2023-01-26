pub mod token;
use std::{ iter::Peekable, vec::IntoIter};

use self::token::{Token, TokenType, LiteralType};

pub struct Lexer<'a> {
    pub src: Peekable<IntoIter<u8>>,
    pub tokens: Vec<Token<'a>>,
    pub line: u16,
}

impl Lexer<'_>{
    pub fn new<'a>(src: String) -> Lexer<'a>{
        let tokens: Vec<Token<'a>> = Vec::new();
        Lexer {
            src: src.into_bytes().into_iter().peekable(),
            tokens,
            line: 1,
        }
    }
    fn add_token(&mut self, ttype: TokenType, literal: LiteralType){
    
        self.tokens.push( Token {
                            ttype,
                            lexeme: "",
                            literal,
                            line: self.line,
                        });
    }
    fn scan_token(&mut self, c: char){
        println!("{}",c );
        match c {
            '-' => self.add_token(TokenType::MINUS, LiteralType::Char('-')),
            '+' => self.add_token(TokenType::PLUS, LiteralType::Char('+')),
            '.' => self.add_token(TokenType::DOT, LiteralType::Char('.')),
            '*' => self.add_token(TokenType::STAR, LiteralType::Char('*')),
            _ => self.syntax_error(self.line, "Found unknown character"),
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

        let eof_tok = Token {       //EOF token
            ttype: TokenType::EOF,
            lexeme: "",
            literal: LiteralType::None(0),      //EOF, lexeme is size 0
            line: self.line,
        };
        self.tokens.push(eof_tok);
    }
    fn syntax_error(&self,line: u16, message: &str){
        eprintln!("[line: {}] ERROR: {}",line,message);
        std::process::exit(exitcode::USAGE)
    }
}
