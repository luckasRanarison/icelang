use crate::tokenizer::tokens::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Token),
    VariableExpression(Token),
    UnaryExpression {
        operator: Token,
        operand: Box<Expression>,
    },
    BinaryExpression {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    BlockExpression(Vec<Statement>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VariableDeclaration {
        token: Token,
        name: String,
        value: Expression,
    },
    VariableAssignement {
        token: Token,
        name: String,
        value: Expression,
    },
    ExpressionStatement(Expression),
}
