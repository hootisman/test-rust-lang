use std::fmt::Debug;

#[derive(Debug)]
pub enum LiteralType {
    Number(i64),
    Char(char),
    String(String),
    None(u32),          //u32 = size of lexeme
}

#[derive(Debug)]
pub enum TokenType {
    MINUS,
    PLUS,
    DOT,
    STAR,
    SEMICOLON,
    LEFTPAREN,
    RIGHTPAREN,
    NUMBER,
    EOF,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub ttype: TokenType,
    pub lexeme: &'a str,
    pub literal: LiteralType,
    pub line: u16,
}

impl Token<'_> {
    pub fn to_string(&self) -> String{
        format!("{:?} {} {:?}", self.ttype, self.lexeme, self.literal)
    }

}
