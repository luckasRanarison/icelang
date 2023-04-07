use super::utils::Position;
use std::fmt;

#[derive(Debug)]
pub enum LexicalError {
    UnexpectedCharacter(String, Position),
    TrailingQuote(char, Position),
    InvalidEscapeChar(String, Position),
    InvalidFloat(String, Position),
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LexicalError::TrailingQuote(ch, pos) => {
                write!(
                    f,
                    "trailing {} at line {}, col {}",
                    ch, pos.line, pos.col_start,
                )
            }
            LexicalError::UnexpectedCharacter(ch, pos) => {
                write!(f, "{}", span_report("unexpected character", ch, pos))
            }
            LexicalError::InvalidFloat(num, pos) => {
                write!(f, "{}", span_report("invalid floating number", num, pos))
            }
            LexicalError::InvalidEscapeChar(str, pos) => {
                write!(f, "{}", span_report("invalid escape character", str, pos))
            }
        }
    }
}

fn span_report(message: &str, str: &String, pos: &Position) -> String {
    format!("{} '{}' at line {}, col {}", message, str, pos.line, {
        match pos.col_start == pos.col_end {
            true => pos.col_start.to_string(),
            false => format!("{}-{}", pos.col_start, pos.col_end),
        }
    })
}
