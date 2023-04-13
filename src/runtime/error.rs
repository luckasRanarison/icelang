use super::value::Value;
use crate::tokenizer::{tokens::Token, utils::Position};
use std::fmt;

pub enum RuntimeError {
    TypeMismatch(String, Position),
    UndefinedVariable(Token),
    RedeclaringVariable(Token),
    ControlFlow(ControlFlow),
}

pub enum ControlFlow {
    Break(Token),
    Continue(Token),
    Return(Value, Token),
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
            RuntimeError::ControlFlow(statement) => match statement {
                ControlFlow::Break(token) | ControlFlow::Continue(token) => write!(
                    f,
                    "unexpected {} statement outside of a loop at line {} col {}",
                    token.lexeme, token.pos.line, token.pos.col_start
                ),
                ControlFlow::Return(_, token) => write!(
                    f,
                    "unexpected return statement at line {} col {}",
                    token.pos.line, token.pos.col_start
                ),
            },
        }
    }
}
