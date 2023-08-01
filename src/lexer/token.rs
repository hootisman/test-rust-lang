use std::fmt::Debug;


#[derive(Debug)]
pub enum TokenType {
    MINUS,
    PLUS,
    EQUAL,
    DOT,
    STAR,
    SEMICOLON,
    LEFTPAREN,
    RIGHTPAREN,

    //literals
    NUMBER(i32),
    IDENTIFER(char),

    EOF,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub ttype: TokenType,
    pub lexeme: &'a str,
    pub line: u16,
}

impl Token<'_> {
    pub fn to_string(&self) -> String{
        format!("line {}, {:?}, \"{}\"",self.line, self.ttype, self.lexeme)
    }

}
