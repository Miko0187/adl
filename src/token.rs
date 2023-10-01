pub enum TokenKind {
    String,
    Number,
    Plus,
    Minus,
    Star,
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