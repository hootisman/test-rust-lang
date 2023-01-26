use std::fmt::Debug;

#[derive(Debug)]
pub enum LiteralType {
    number(i64),
    char(char),
    None,
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
