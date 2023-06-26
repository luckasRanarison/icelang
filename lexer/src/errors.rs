use std::fmt;

use super::utils::Position;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct LexicalError {
    pub kind: LexicalErrorKind,
    pub position: Position,
}

impl LexicalError {
    pub fn new(kind: LexicalErrorKind, position: Position) -> Self {
        Self { kind, position }
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.kind, self.position)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum LexicalErrorKind {
    #[error("unexpected character '{0}'")]
    UnexpectedCharacter(String),
    #[error("trailing quote {0}")]
    TrailingQuote(char),
    #[error("invalid escape character '{0}'")]
    InvalidEscapeChar(String),
    #[error("invalid foating number '{0}'")]
    InvalidFloat(String),
}
