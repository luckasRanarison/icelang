use super::utils::Position;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexicalError {
    #[error("unexpected character '{0}' at {1}")]
    UnexpectedCharacter(String, Position),
    #[error("trailing quote {0} at {1}")]
    TrailingQuote(char, Position),
    #[error("invalid escape character '{0}' at {1}")]
    InvalidEscapeChar(String, Position),
    #[error("invalid foating number '{0}' at {1}")]
    InvalidFloat(String, Position),
}
