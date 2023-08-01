pub mod token;
use std::{ iter::Peekable, vec::IntoIter};

use self::token::{Token, TokenType };

pub struct Lexer<'a> {
    pub src: Peekable<IntoIter<u8>>,
    pub tokens: Vec<Token<'a>>,
    pub line: u16,
}

impl<'a> Lexer<'a>{
    pub fn new<'b>(src: String) -> Lexer<'b>{
        //converts src string to peekable
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
    fn number_token(&mut self, num: char){
        //creates number literal by adding all chars that form the number to a
        //string, then converting the String to a i32
        //ex: "2323423" becomes 2323423 i32
        let mut literal = String::new();
        literal.push(num); //first char

        loop {
            match self.src.peek(){
                Some(c) if (*c as char).is_digit(10) => {
                    //next char is a number, add src.next() to 'literal'
                    literal.push(self.src.next().unwrap() as char);
                },
                Some(_) | None => break,
            }
        }
        self.add_token(TokenType::NUMBER(literal.parse::<i32>().unwrap()), "");
    }
    fn scan_token(&mut self, c: char){
        // println!("{}",c );
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
            num @ '0'..='9' => self.number_token(num),
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
