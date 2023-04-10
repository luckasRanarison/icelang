use crate::tokenizer::tokens::Token;
use std::fmt;

#[derive(Debug)]
pub enum ParsingError {
    UnexpectedToken(Token),
    UnexpedtedEndOfInput(Token),
    MissingParenthese(Token),
    MissingLeftOperand(Token),
    MissingRightOperand(Token),
    MissingInitializer(Token),
    MissingSemicolon(Token),
    MissingAssignment(Token),
    MissingClosingBrace(Token),
    ExpectedIdentifier(Token),
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::UnexpectedToken(token) => write!(
                f,
                "unexpected token '{}' at line {} col {}",
                token.lexeme, token.pos.line, token.pos.col_start,
            ),
            ParsingError::UnexpedtedEndOfInput(token) => write!(
                f,
                "unexpected end of input at line {} col {}",
                token.pos.line, token.pos.col_end
            ),
            ParsingError::MissingParenthese(token)
            | ParsingError::MissingLeftOperand(token)
            | ParsingError::MissingRightOperand(token)
            | ParsingError::MissingInitializer(token)
            | ParsingError::MissingSemicolon(token)
            | ParsingError::MissingAssignment(token)
            | ParsingError::MissingClosingBrace(token) => write!(
                f,
                "missing {} at line {} pos {}",
                match self {
                    ParsingError::MissingParenthese(_) => "closing ')'",
                    ParsingError::MissingLeftOperand(_) => "left operand",
                    ParsingError::MissingRightOperand(_) => "right operand",
                    ParsingError::MissingInitializer(_) => "initializer",
                    ParsingError::MissingSemicolon(_) => "semicolon ';'",
                    ParsingError::MissingAssignment(_) => "assignment '='",
                    ParsingError::MissingClosingBrace(_) => "closing brace for '{'",
                    _ => unreachable!(),
                },
                token.pos.line,
                token.pos.col_start,
            ),
            ParsingError::ExpectedIdentifier(token) => write!(
                f,
                "expected identifier but found '{}' at line {} col {}",
                token.lexeme, token.pos.line, token.pos.col_start
            ),
        }
    }
}
