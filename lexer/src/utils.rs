use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub line_start: u32,
    pub line_end: u32,
    pub col_start: u32,
    pub col_end: u32,
}

impl Position {
    pub fn new(line_start: u32, col_start: u32, line_end: u32, col_end: u32) -> Self {
        Self {
            line_start,
            col_start,
            line_end,
            col_end,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let col = match self.col_start == self.col_end {
            true => (self.col_start + 1).to_string(),
            false => format!("{}-{}", self.col_start, self.col_end),
        };

        let line = match self.line_start == self.line_end {
            true => (self.line_start + 1).to_string(),
            false => format!("{}-{}", self.line_start, self.line_end),
        };

        write!(f, "line {}, col {}", line, col)
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
