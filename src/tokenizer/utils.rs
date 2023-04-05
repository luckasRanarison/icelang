#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl Position {
    pub fn new(line: usize, start: usize, end: usize) -> Self {
        Self { line, start, end }
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
