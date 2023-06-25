use lexer::{errors::LexicalError, tokens::Token};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("unexpected token '{}' ({})", .0.lexeme, .0.pos)]
    UnexpectedToken(Token),
    #[error("unexpected end of input ({})", .0.pos)]
    UnexpedtedEndOfInput(Token),
    #[error("missing closing parenthesis ({})", .0.pos)]
    MissingParenthesis(Token),
    #[error("missing left operand for '{}' ({})", .0.lexeme, .0.pos)]
    MissingLeftOperand(Token),
    #[error("missing right operand for '{}' ({})", .0.lexeme, .0.pos)]
    MissingRightOperand(Token),
    #[error("missing variable initializer ({})", .0.pos)]
    MissingInitializer(Token),
    #[error("missing semicolon ';' ({})", .0.pos)]
    MissingSemicolon(Token),
    #[error("missing assignment '=' ({})", .0.pos)]
    MissingAssignment(Token),
    #[error("expected colon ':' but got '{}' ({})", .0.lexeme, .0.pos)]
    ExpectedColon(Token),
    #[error("expected comma ',' but got '{}' ({})", .0.lexeme, .0.pos)]
    ExpectedComma(Token),
    #[error("expected left brace '{{' but got '{}' ({})", .0.lexeme, .0.pos)]
    ExpectedLeftBrace(Token),
    #[error("expected left parenthesis '(' but got '{}' ({})", .0.lexeme, .0.pos)]
    ExpectedLeftParenthesis(Token),
    #[error("expected parameter name but got '{}' ({})", .0.lexeme, .0.pos)]
    ExpectedParameter(Token),
    #[error("missing closing bracket ']' ({})", .0.pos)]
    MissingClosingBracket(Token),
    #[error("missing closing brace '}}' ({})", .0.pos)]
    MissingClosingBrace(Token),
    #[error("expected identifer but got '{}' ({})", .0.lexeme, .0.pos)]
    ExpectedIdentifier(Token),
    #[error("expected 'in' but got '{}' ({})", .0.lexeme, .0.pos)]
    ExpectedIn(Token),
    #[error("missing match arm expression ({})", .0.pos)]
    MissingArmExpression(Token),
    #[error("missing comma ',' ({})", .0.pos)]
    MissingComma(Token),
    #[error("invalid assignment target ({})", .0.pos)]
    InvalidAssignment(Token),
    #[error("invalid property name ({})", .0.pos)]
    InvalidProp(Token),
    #[error("{}", .0)]
    LexicalError(LexicalError),
}
