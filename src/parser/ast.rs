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
    IfExpression {
        condition: Box<Expression>,
        true_branch: Box<Expression>,
        else_branch: Option<Box<Expression>>,
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
            Expression::IfExpression {
                condition,
                true_branch,
                else_branch,
            } => {
                let mut else_str = String::new();
                if let Some(else_branch) = else_branch {
                    else_str.push_str(&format!(" else {}", *else_branch))
                }

                write!(f, "if ({}) {}{}", condition, true_branch, else_str)
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
