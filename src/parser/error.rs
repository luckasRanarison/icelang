use crate::tokenizer::tokens::Token;
use std::fmt;

#[derive(Debug)]
pub enum ParsingError {
    NotYetImplemented,
    UnexpectedToken(Token),
    MissingParenthese(Token),
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::NotYetImplemented => write!(f, "not yet impemented"),
            ParsingError::UnexpectedToken(token) => write!(
                f,
                "unexpected token '{}' at line {} col:{}",
                token.lexeme, token.pos.line, token.pos.col_start
            ),
            ParsingError::MissingParenthese(token) => write!(
                f,
                "missing closing ')' at line {} pos {}",
                token.pos.line, token.pos.col_start
            ),
        }
    }
}
