use crate::lexer::tokens::Token;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("unexpected token '{}' at {}", .0.lexeme, .0.pos)]
    UnexpectedToken(Token),
    #[error("unexpected end of input at {}", .0.pos)]
    UnexpedtedEndOfInput(Token),
    #[error("missing closing parenthesis at '{}'", .0.pos)]
    MissingParenthesis(Token),
    #[error("missing left operand for '{}' at '{}'", .0.lexeme, .0.pos)]
    MissingLeftOperand(Token),
    #[error("missing right operand for '{}' at '{}'", .0.lexeme, .0.pos)]
    MissingRightOperand(Token),
    #[error("missing variable initializer at '{}'", .0.pos)]
    MissingInitializer(Token),
    #[error("missing semicolon ';' before '{}'", .0.pos)]
    MissingSemicolon(Token),
    #[error("missing assignment '=' at '{}'", .0.pos)]
    MissingAssignment(Token),
    #[error("expected colon ':' but got '{}' at {}", .0.lexeme, .0.pos)]
    ExpectedColon(Token),
    #[error("expected comma ',' but got '{}' at {}", .0.lexeme, .0.pos)]
    ExpectedComma(Token),
    #[error("expected left brace '{{' but got '{}' at {}", .0.lexeme, .0.pos)]
    ExpectedLeftBrace(Token),
    #[error("expected left parenthesis '(' but got '{}' at {}", .0.lexeme, .0.pos)]
    ExpectedLeftParenthesis(Token),
    #[error("expected parameter name but got '{}' at {}", .0.lexeme, .0.pos)]
    ExpectedParameter(Token),
    #[error("missing closing brace '}}' at '{}'", .0.pos)]
    MissingClosingBracket(Token),
    #[error("missing closing bracket ']' at '{}'", .0.pos)]
    MissingClosingBrace(Token),
    #[error("expected identifer but got '{}' at {}", .0.lexeme, .0.pos)]
    ExpectedIdentifier(Token),
    #[error("missing match arm expression at line {}", .0.pos)]
    MissingArmExpression(Token),
    #[error("missing comma ',' at line '{}'", .0.pos)]
    MissingComma(Token),
    #[error("invalid assignment target at line {}", .0.pos)]
    InvalidAssignment(Token),
    #[error("invalid property name at line {}", .0.pos)]
    InvalidProp(Token),
}
