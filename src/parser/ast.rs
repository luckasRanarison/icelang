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
    BlockExpression {
        statements: Vec<Statement>,
        return_expr: Option<Box<Expression>>,
    },
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
            Expression::BlockExpression {
                statements,
                return_expr,
            } => {
                let mut s = String::new();

                for statement in statements {
                    s.push_str(&format!(" {};", *statement));
                }

                if let Some(return_expr) = return_expr {
                    s.push_str(&format!(" {}", *return_expr));
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
    WhileStatement {
        condition: Box<Expression>,
        block: Box<Expression>,
    },
    BreakStatement(Token),
    ContinueStatement(Token),
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
            Statement::WhileStatement { condition, block } => {
                write!(f, "while ({}) {}", condition, *block)
            }
            Statement::BreakStatement(_) => write!(f, "break"),
            Statement::ContinueStatement(_) => write!(f, "continue"),
        }
    }
}
