use super::value::Value;
use crate::{
    lexer::{errors::LexicalError, tokens::Token, utils::Position},
    parser::error::ParsingError,
};
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
    #[error("trying to index an unindexable object at {}", .0.pos)]
    UnindexableType(Token),
    #[error("invalid index value at {}", .0.pos)]
    InvalidIndex(Token),
    #[error("trying to call a non function expression at {}", .0.pos)]
    NotFunciton(Token),
    #[error("trying to assign index value to a non-array variable at {}", .0.pos)]
    NotAnArray(Token),
    #[error("calling property on a non-object type at {}", .0.pos)]
    NotAnObject(Token),
    #[error("expected {0} argument but got {1} at {}", .2.pos)]
    InvalidArgument(usize, usize, Token),
    #[error("invalid assignment at line {}", .0.pos)]
    InvalidAssignment(Token),
    #[error("module '{0}' not found at line {}", .1.pos)]
    ModuleNotFound(String, Token),
    #[error("invalid path '{0}' at line {}", .1.pos)]
    InvalidPath(Value, Token),
    #[error("{0}")]
    LexicalError(LexicalError),
    #[error("{0}")]
    ParsingError(ParsingError),
    #[error("cannot export module in REPL mode")]
    Export(Value),
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
