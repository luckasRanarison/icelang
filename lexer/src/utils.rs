use std::fmt;

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

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let col = if self.col_start == self.col_end {
            self.col_start.to_string()
        } else {
            format!("{}-{}", self.col_start, self.col_end)
        };

        write!(f, "line {}, col {}", self.line, col)
    }
}

pub fn is_standard_symbol(ch: char) -> bool {
    let symbols = "+-%*/(){}[].,;:!<>=";
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
