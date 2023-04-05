use super::tokens::Position;
use std::fmt;

#[derive(Debug)]
pub enum LexicalError {
    UnexpectedCharacter(String, Position),
    MismatchedParentheses(Position),
    InvalidFloat(String, Position),
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LexicalError::UnexpectedCharacter(ch, pos) => {
                write!(
                    f,
                    "unexpected character: '{}' at line {}, col {}-{}",
                    ch, pos.line, pos.start, pos.end
                )
            }
            LexicalError::MismatchedParentheses(pos) => write!(
                f,
                "mismatched parentheses at line {}, col {}-{}",
                pos.line, pos.start, pos.end,
            ),
            LexicalError::InvalidFloat(num, pos) => write!(
                f,
                "invalid number: '{}' at line {}, col {}-{}",
                num, pos.line, pos.start, pos.end,
            ),
        }
    }
}
