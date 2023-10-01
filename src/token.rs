use std::fmt;

pub enum TokenKind {
    String,
    Number,
    Plus,
    Minus,
    Multiply,
    Slash,
    LParen,
    RParen,
    Equal,
    Semicolon,
    Unknown,
}

pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Token {
        Token {
            kind: kind,
            value: value,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}