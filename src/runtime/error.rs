use crate::tokenizer::{tokens::Token, utils::Position};
use std::fmt;

pub enum RuntimeError {
    TypeMismatch(String, Position),
    UndefinedVariable(Token),
    RedeclaringVariable(Token),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::TypeMismatch(message, pos) => write!(
                f,
                "type mismatch: {} at line {} col {}, ",
                message, pos.line, pos.col_start
            ),
            RuntimeError::UndefinedVariable(variable) => write!(
                f,
                "undefined variable '{}' at line {} col {}",
                variable.lexeme, variable.pos.line, variable.pos.col_start
            ),
            Self::RedeclaringVariable(variable) => write!(
                f,
                "redeclaring existing variable '{}' at line {} col {}",
                variable.lexeme, variable.pos.line, variable.pos.col_start
            ),
        }
    }
}
