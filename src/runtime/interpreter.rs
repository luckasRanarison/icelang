use std::process;

use super::{error::RuntimeError, value::Value};
use crate::{
    parser::ast::{Expression, Statement},
    tokenizer::tokens::{Token, TokenType},
};

pub struct Interpreter {}

impl Interpreter {
    pub fn visit_nodes(&self, nodes: &Vec<Statement>) {
        for node in nodes {
            let value = match node {
                Statement::ExpressionStatement(expr) => self.evaluate_expression(expr),
                _ => todo!(),
            };

            let value = value.unwrap_or_else(|err| {
                eprintln!("{err}");
                process::exit(1);
            });

            println!("{}", value.stringify());
        }
    }

    fn evaluate_expression(&self, expr: &Expression) -> Result<Value, RuntimeError> {
        match expr {
            Expression::Literal(literal) => Ok(self.evaluate_literal(literal)),
            Expression::UnaryExpression { operator, operand } => {
                Ok(self.evaluate_unary(operator, operand)?)
            }
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => Ok(self.evaluate_binary(left, operator, right)?),
        }
    }

    fn evaluate_literal(&self, literal: &Token) -> Value {
        match &literal.value {
            TokenType::Number(value) => Value::Number(*value),
            TokenType::String(value) => Value::String(value.clone()),
            TokenType::True => Value::Boolean(true),
            TokenType::False => Value::Boolean(false),
            _ => Value::Null,
        }
    }

    fn evaluate_unary(
        &self,
        operator: &Token,
        operand: &Expression,
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
        &self,
        left: &Expression,
        operator: &Token,
        right: &Expression,
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
            _ => unreachable!(),
        }
    }
}
