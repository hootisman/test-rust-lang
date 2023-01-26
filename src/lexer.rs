pub mod token;
use self::token::{Token, TokenType, LiteralType};

pub struct Lexer<'a> {
    pub src: String,
    pub tokens: Vec<Token<'a>>,
    pub start: u16,
    pub current: u16,
    pub line: u16,
}

impl Lexer<'_>{
    pub fn new<'a>(src: String) -> Lexer<'a>{
        let tokens: Vec<Token<'a>> = Vec::new();
        Lexer {
            src,
            tokens,
            start: 0,
            current: 0,
            line: 1,
        }
    }
    fn is_at_end(&self) -> bool{
        let length: u16 = self.src.len() as u16;
        self.current >= length
    }
    fn scan_token(&mut self){

    }
    pub fn scan_tokens(&mut self){
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let eof_tok = Token {
            ttype: TokenType::EOF,
            lexeme: "",
            literal: LiteralType::None,
            line: self.line,
        };
        self.tokens.push(eof_tok);
    }
}
