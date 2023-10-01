use crate::token::{
    Token,
    TokenKind,
};

pub fn lexer(data: &String) -> Vec<Token> {
    let mut pos = 0;
    let mut tokens: Vec<Token> = Vec::new();

    while pos < data.len() {
        let c = data.chars().nth(pos).unwrap();

        if c.is_whitespace() {
            pos += 1;
            continue;
        }
        else if c.is_ascii_alphabetic() {
            let string = get_string(data, pos);
            pos += string.len();
            add_token(&mut tokens, TokenKind::String, string);
            continue;
        }
        else if c.is_ascii_digit() {
            let number = get_number(data, pos);
            pos += number.len();
            add_token(&mut tokens, TokenKind::Number, number);
            continue;
        }
        else {            
            match c {
                '+' => add_token(&mut tokens, TokenKind::Plus, c.to_string()),
                '-' => add_token(&mut tokens, TokenKind::Minus, c.to_string()),
                '*' => add_token(&mut tokens, TokenKind::Multiply, c.to_string()),
                '/' => add_token(&mut tokens, TokenKind::Slash, c.to_string()),
                '(' => add_token(&mut tokens, TokenKind::LParen, c.to_string()),
                ')' => add_token(&mut tokens, TokenKind::RParen, c.to_string()),
                '=' => add_token(&mut tokens, TokenKind::Equal, c.to_string()),
                ';' => add_token(&mut tokens, TokenKind::Semicolon, c.to_string()),
                _ => add_token(&mut tokens, TokenKind::Unknown, c.to_string()),
            }
        }

        pos += 1;
    }

    return tokens;
}

fn get_string(data: &String, pos: usize) -> String {
    let mut string = String::new();
    let mut pos = pos;

    while pos < data.len() {
        let c = data.chars().nth(pos).unwrap();
        if !c.is_ascii_alphabetic() {
            break;
        }
        string.push(c);
        pos += 1;
    }

    string
}

fn get_number(data: &String, pos: usize) -> String {
    let mut number = String::new();
    let mut pos = pos;

    while pos < data.len() {
        let c = data.chars().nth(pos).unwrap();
        if !c.is_ascii_digit() {
            break;
        }
        number.push(c);
        pos += 1;
    }

    number
}

fn add_token(tokens: &mut Vec<Token>, kind: TokenKind, c: String) {
    tokens.push(Token::new(kind, c));
}
