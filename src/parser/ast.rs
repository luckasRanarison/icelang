use crate::tokenizer::tokens::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Token),
    UnaryExpression {
        operator: Token,
        operand: Box<Expression>,
    },
    BinaryExpression {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
}
