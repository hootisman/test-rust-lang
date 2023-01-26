pub mod token;

pub struct Lexer<'a> {
    pub src: String,
    pub tokens: Vec<self::token::Token<'a>>,
}

impl Lexer<'_>{
    pub fn new<'a>(src: String) -> Lexer<'a>{
        let tokens: Vec<self::token::Token<'a>> = Vec::new();
        Lexer {
            src,
            tokens,
        }
    }
}
