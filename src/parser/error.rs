use crate::tokenizer::tokens::Token;
use std::fmt;

#[derive(Debug)]
pub enum ParsingError {
    UnexpectedToken(Token),
    MissingParenthese(Token),
    MissingLeftOperand(Token),
    MissingRightOperand(Token),
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::UnexpectedToken(token) => write!(
                f,
                "unexpected token '{}' at line {} col {}",
                token.lexeme, token.pos.line, token.pos.col_start,
            ),
            ParsingError::MissingParenthese(token)
            | ParsingError::MissingLeftOperand(token)
            | ParsingError::MissingRightOperand(token) => write!(
                f,
                "missing {} at line {} pos {}",
                match self {
                    ParsingError::MissingParenthese(_) => "closing ')'",
                    ParsingError::MissingLeftOperand(_) => "left operand",
                    ParsingError::MissingRightOperand(_) => "right operand",
                    _ => unreachable!(),
                },
                token.pos.line,
                token.pos.col_start,
            ),
        }
    }
}
