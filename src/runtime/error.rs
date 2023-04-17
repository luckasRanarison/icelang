use super::value::Value;
use crate::lexer::{tokens::Token, utils::Position};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("expected '{0}', but found '{1}' at {2}")]
    TypeExpection(String, String, Position),
    #[error("{0} at {1}")]
    InvalidOperation(String, Position),
    #[error("undefined variable '{}' at {}", .0.lexeme, .0.pos)]
    UndefinedVariable(Token),
    #[error("redeclaring existing identifier '{}' at {}", .0.lexeme, .0.pos)]
    RedeclaringIdentifier(Token),
    #[error("{0}")]
    ControlFlow(ControlFlow),
    #[error("only array and string can be indexed")]
    UnindexableType,
    #[error("only positive number can be used to index array")]
    InvalidIndex,
    #[error("not a funciton")]
    NotFunciton,
    #[error("expected {0} argument but got {1}")]
    InvalidArgument(usize, usize),
}

#[derive(Debug, Error)]
pub enum ControlFlow {
    #[error("unexpected break statement outside of a loop at {}", .0.pos)]
    Break(Token),
    #[error("unexpected continue statement outside of a loop at {}", .0.pos)]
    Continue(Token),
    #[error("unexpected return statement outside of a function at {}", .1.pos)]
    Return(Value, Token),
}
