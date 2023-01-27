use std::fmt::Debug;


#[derive(Debug)]
pub enum TokenType {
    MINUS,
    PLUS,
    DOT,
    STAR,
    SEMICOLON,
    LEFTPAREN,
    RIGHTPAREN,

    //literals
    NUMBER(u32),
    IDENTIFER(char),

    EOF,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub ttype: TokenType,
    pub lexeme: &'a str,
    // pub literal: LiteralType,
    pub line: u16,
}

impl Token<'_> {
    pub fn to_string(&self) -> String{
        // format!("{:?}, \"{}\", {:?}", self.ttype, self.lexeme, self.literal)
        format!("line {}, {:?}, \"{}\"",self.line, self.ttype, self.lexeme)
    }

}
