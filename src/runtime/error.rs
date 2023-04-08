use std::fmt;

use crate::tokenizer::utils::Position;

pub enum RuntimeError {
    TypeMismatch(String, Position),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::TypeMismatch(message, pos) => write!(
                f,
                "type mismatch: {} at line {} col {}, ",
                message, pos.line, pos.col_start
            ),
        }
    }
}
