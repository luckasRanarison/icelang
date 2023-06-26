use std::fmt;

use super::value::Value;
use lexer::{errors::LexicalErrorKind, utils::Position};
use parser::error::ParsingErrorKind;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
    pub position: Position,
}

impl RuntimeError {
    pub fn new(kind: RuntimeErrorKind, position: Position) -> Self {
        Self { kind, position }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.kind, self.position)
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum RuntimeErrorKind {
    #[error("expected '{0}', but found '{1}'")]
    TypeExpection(String, String),
    #[error("{0}")]
    InvalidOperation(String),
    #[error("division by zero")]
    DivisionByZero,
    #[error("undefined identifier '{0}'")]
    UndefinedIdentifier(String),
    #[error("redeclaring existing identifier '{0}'")]
    RedeclaringIdentifier(String),
    #[error("{0}")]
    ControlFlow(ControlFlow),
    #[error("trying to index an unindexable object")]
    UnindexableType,
    #[error("invalid index value")]
    InvalidIndex,
    #[error("trying to call a non function expression")]
    NotFunciton,
    #[error("trying to assign index value to a non-array variable)")]
    NotAnArray,
    #[error("calling property on a non-object type)")]
    NotAnObject,
    #[error("expected {0} argument but got {1}")]
    InvalidArgument(usize, usize),
    #[error("invalid assignment")]
    InvalidAssignment,
    #[error("invalid range")]
    InvalidRange,
    #[error("invalid number parsing")]
    InvalidNumber,
    #[error("module '{0}' not found")]
    ModuleNotFound(String),
    #[error("non-iterable type")]
    NonIterable,
    #[error("invalid argument type")]
    InvalidArg,
    #[error("mismatched arguments type")]
    MismatchedArg,
    #[error("invalid path '{0}'")]
    InvalidPath(Value),
    #[error("{0}")]
    LexicalError(LexicalErrorKind),
    #[error("{0}")]
    ParsingError(ParsingErrorKind),
    #[error("cannot export module in REPL mode")]
    Export(Value),
}

#[derive(Debug, Error, PartialEq)]
pub enum ControlFlow {
    #[error("unexpected break statement outside of a loop")]
    Break,
    #[error("unexpected continue statement outside of a loop")]
    Continue,
    #[error("unexpected return statement outside of a function")]
    Return(Value),
}
