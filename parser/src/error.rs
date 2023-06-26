use std::fmt;

use lexer::{errors::LexicalErrorKind, utils::Position};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct ParsingError {
    pub kind: ParsingErrorKind,
    pub position: Position,
}

impl ParsingError {
    pub fn new(kind: ParsingErrorKind, position: Position) -> Self {
        Self { kind, position }
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.kind, self.position)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum ParsingErrorKind {
    #[error("unexpected token '{0}'")]
    UnexpectedToken(String),
    #[error("unexpected end of input")]
    UnexpedtedEndOfInput,
    #[error("missing closing parenthesis")]
    MissingParenthesis,
    #[error("missing left operand for '{0}'")]
    MissingLeftOperand(String),
    #[error("missing right operand for '{0}'")]
    MissingRightOperand(String),
    #[error("missing variable initializer")]
    MissingInitializer,
    #[error("missing semicolon ';'")]
    MissingSemicolon,
    #[error("missing assignment '='")]
    MissingAssignment,
    #[error("expected colon ':' but got '{0}'")]
    ExpectedColon(String),
    #[error("expected comma ',' but got '{0}'")]
    ExpectedComma(String),
    #[error("expected left brace '{{' but got '{0}'")]
    ExpectedLeftBrace(String),
    #[error("expected left parenthesis '(' but got '{0}'")]
    ExpectedLeftParenthesis(String),
    #[error("expected parameter name but got '{0}'")]
    ExpectedParameter(String),
    #[error("missing closing bracket ']'")]
    MissingClosingBracket,
    #[error("missing closing brace '}}'")]
    MissingClosingBrace,
    #[error("expected identifer but got '{0}'")]
    ExpectedIdentifier(String),
    #[error("expected 'in' but got '{0}'")]
    ExpectedIn(String),
    #[error("missing match arm expression")]
    MissingArmExpression,
    #[error("missing comma ','")]
    MissingComma,
    #[error("invalid assignment target")]
    InvalidAssignment,
    #[error("invalid property name '{0}'")]
    InvalidProp(String),
    #[error("{0}")]
    LexicalError(LexicalErrorKind),
}
