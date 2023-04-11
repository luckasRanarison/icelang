use crate::tokenizer::tokens::Token;
use std::fmt;

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

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(token) => write!(f, "{}", token.lexeme),
            Expression::UnaryExpression { operator, operand } => {
                write!(f, "({}{})", operator.lexeme, *operand)
            }
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", *left, operator.lexeme, *right),
            Expression::VariableExpression(token) => write!(f, "{}", token.lexeme),
            Expression::BlockExpression(statements) => {
                let mut s = String::new();

                for statement in statements {
                    s.push_str(&format!(" {};", *statement));
                }

                write!(f, "{{{} }}", s)
            }
        }
    }
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

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::VariableDeclaration {
                token: _,
                name,
                value,
            } => write!(f, "set {} = {}", name, *value),
            Statement::VariableAssignement {
                token: _,
                name,
                value,
            } => write!(f, "{} = {}", name, *value),
            Statement::ExpressionStatement(expr) => write!(f, "{expr}"),
        }
    }
}
