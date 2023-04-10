use super::{environment::Environment, error::RuntimeError, value::Value};
use crate::{
    parser::ast::{Expression, Statement},
    tokenizer::tokens::{Token, TokenType},
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn evaluate_statement(&mut self, node: Statement) -> Result<Value, RuntimeError> {
        let value = match node {
            Statement::VariableDeclaration { token, name, value } => {
                if self.environment.contains(&name) {
                    return Err(RuntimeError::RedeclaringVariable(token));
                }

                let value = self.evaluate_expression(value)?;
                self.environment.store(name, value.clone());
                value
            }
            Statement::VariableAssignement { token, name, value } => {
                let value = self.evaluate_expression(value)?;

                if !self.environment.contains(&name) {
                    return Err(RuntimeError::UndefinedVariable(token));
                }

                self.environment.store(name, value.clone());
                value
            }
            Statement::ExpressionStatement(expr) => self.evaluate_expression(expr)?,
        };

        Ok(value)
    }

    fn evaluate_expression(&mut self, expr: Expression) -> Result<Value, RuntimeError> {
        match expr {
            Expression::Literal(literal) => Ok(self.evaluate_literal(literal)),
            Expression::VariableExpression(variable) => Ok(self.evaluate_variable(variable)?),
            Expression::UnaryExpression { operator, operand } => {
                Ok(self.evaluate_unary(operator, *operand)?)
            }
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => Ok(self.evaluate_binary(*left, operator, *right)?),
            Expression::BlockExpression(statements) => self.evaluate_block(statements),
        }
    }

    fn evaluate_variable(&self, variable: Token) -> Result<Value, RuntimeError> {
        match self.environment.get(variable.lexeme.clone()) {
            Some(value) => Ok(value),
            None => Err(RuntimeError::UndefinedVariable(variable.clone())),
        }
    }

    fn evaluate_block(&mut self, statements: Vec<Statement>) -> Result<Value, RuntimeError> {
        let prev = self.environment.clone();
        self.environment = Environment::from(prev.clone());

        if statements.is_empty() {
            return Ok(Value::Null);
        }

        for statement in &statements {
            self.evaluate_statement(statement.clone())?;
        }

        let value = match statements.last().unwrap() {
            Statement::ExpressionStatement(expr) => self.evaluate_expression(expr.to_owned())?,
            _ => Value::Null,
        };

        self.environment = prev;

        Ok(value)
    }

    fn evaluate_literal(&self, literal: Token) -> Value {
        match &literal.value {
            TokenType::Number(value) => Value::Number(*value),
            TokenType::String(value) => Value::String(value.clone()),
            TokenType::True => Value::Boolean(true),
            TokenType::False => Value::Boolean(false),
            _ => Value::Null,
        }
    }

    fn evaluate_unary(
        &mut self,
        operator: Token,
        operand: Expression,
    ) -> Result<Value, RuntimeError> {
        let right = self.evaluate_expression(operand)?;

        match operator.value {
            TokenType::Minus => match right {
                Value::Number(value) => Ok(Value::Number(-value)),
                _ => Err(RuntimeError::TypeMismatch(
                    format!("expected 'number' but found '{}'", right.get_type()),
                    operator.pos,
                )),
            },
            TokenType::Bang => match right {
                Value::Boolean(value) => Ok(Value::Boolean(!value)),
                _ => Ok(Value::Boolean(true)),
            },
            _ => unreachable!(),
        }
    }

    fn evaluate_binary(
        &mut self,
        left: Expression,
        operator: Token,
        right: Expression,
    ) -> Result<Value, RuntimeError> {
        let lhs = self.evaluate_expression(left)?;
        let rhs = self.evaluate_expression(right)?;
        let left_type = lhs.get_type();
        let right_type = rhs.get_type();

        match operator.value {
            TokenType::Asterix => match lhs * rhs {
                Some(value) => Ok(value),
                None => Err(RuntimeError::TypeMismatch(
                    format!("can't multiply a '{}' by a '{}'", left_type, right_type),
                    operator.pos,
                )),
            },
            TokenType::Slash => match lhs / rhs {
                Some(value) => Ok(value),
                None => Err(RuntimeError::TypeMismatch(
                    format!("can't divide a '{}' by a '{}'", left_type, right_type),
                    operator.pos,
                )),
            },
            TokenType::Minus => match lhs - rhs {
                Some(value) => Ok(value),
                None => Err(RuntimeError::TypeMismatch(
                    format!("can't substract a '{}' by a '{}'", left_type, right_type),
                    operator.pos,
                )),
            },
            TokenType::Plus => match lhs + rhs {
                Some(value) => Ok(value),
                None => Err(RuntimeError::TypeMismatch(
                    format!("can't add a '{}' by a '{}'", left_type, right_type),
                    operator.pos,
                )),
            },
            TokenType::Greater => Ok(Value::Boolean(lhs > rhs)),
            TokenType::GreaterEqual => Ok(Value::Boolean(lhs >= rhs)),
            TokenType::Less => Ok(Value::Boolean(lhs < rhs)),
            TokenType::LessEqual => Ok(Value::Boolean(lhs <= rhs)),
            TokenType::EqualEqual => Ok(Value::Boolean(lhs == rhs)),
            TokenType::BangEqual => Ok(Value::Boolean(lhs != rhs)),
            _ => unreachable!(),
        }
    }
}
