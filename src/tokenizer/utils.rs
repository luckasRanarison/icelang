use super::tokens::{Token, TokenType};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub col_start: usize,
    pub col_end: usize,
}

impl Position {
    pub fn new(line: usize, col_start: usize, col_end: usize) -> Self {
        Self {
            line,
            col_start,
            col_end,
        }
    }
}

pub fn is_standard_symbol(ch: char) -> bool {
    let symbols = "+-*/(){}.,;!<>=";
    symbols.contains(ch)
}

pub fn is_quote(ch: char) -> bool {
    ch == '\"' || ch == '\''
}

pub fn is_alphabetic(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

pub fn is_alphanumeric(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

pub fn create_eof_token() -> Token {
    let eof_token = Token::new(TokenType::Eof, String::new(), Position::new(1, 0, 0));

    eof_token
}
