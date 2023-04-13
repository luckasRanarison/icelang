use super::{
    environment::Environment,
    error::{ControlFlow, RuntimeError},
    value::Value,
};
use crate::{
    parser::ast::{Expression, Statement},
    tokenizer::tokens::{Token, TokenType},
};

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn evaluate_statement(&mut self, node: Statement) -> Result<Option<Value>, RuntimeError> {
        let value = match node {
            Statement::VariableDeclaration { token, name, value } => {
                if self.environment.local_contains(&name) {
                    return Err(RuntimeError::RedeclaringVariable(token));
                }

                let value = self.evaluate_expression(value)?;
                self.environment.store(name, value.clone());
                None
            }
            Statement::VariableAssignement { token, name, value } => {
                if !self.environment.global_contains(&name) {
                    return Err(RuntimeError::UndefinedVariable(token));
                }

                let value = self.evaluate_expression(value)?;
                self.environment.assign(name, value.clone());
                None
            }
            Statement::ExpressionStatement(expr) => Some(self.evaluate_expression(expr)?),
            Statement::WhileStatement { condition, block } => {
                self.evaluate_while_statement(*condition, *block)?;
                None
            }
            Statement::BreakStatement(token) => {
                return Err(RuntimeError::ControlFlow(ControlFlow::Break(token)))
            }
            Statement::ContinueStatement(token) => {
                return Err(RuntimeError::ControlFlow(ControlFlow::Continue(token)));
            }
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
            Expression::BlockExpression {
                statements,
                return_expr,
            } => Ok(self.evaluate_block(statements, return_expr)?),
            Expression::IfExpression {
                condition,
                true_branch,
                else_branch,
            } => Ok(self.evaluate_if_expression(condition, true_branch, else_branch)?),
        }
    }

    fn evaluate_variable(&self, variable: Token) -> Result<Value, RuntimeError> {
        match self.environment.get(variable.lexeme.clone()) {
            Some(value) => Ok(value),
            None => Err(RuntimeError::UndefinedVariable(variable.clone())),
        }
    }

    fn evaluate_block(
        &mut self,
        statements: Vec<Statement>,
        return_expr: Option<Box<Expression>>,
    ) -> Result<Value, RuntimeError> {
        self.environment = Environment::from(self.environment.clone());

        for statement in &statements {
            match statement {
                Statement::BreakStatement(token) => {
                    return Err(RuntimeError::ControlFlow(ControlFlow::Break(token.clone())))
                }
                Statement::ContinueStatement(token) => {
                    return Err(RuntimeError::ControlFlow(ControlFlow::Continue(
                        token.clone(),
                    )))
                }
                _ => self.evaluate_statement(statement.clone())?,
            };
        }

        let value = match return_expr {
            Some(expr) => self.evaluate_expression(*expr)?,
            None => Value::Null,
        };

        if let Some(enclosing) = self.environment.enclosing.as_mut() {
            self.environment = *enclosing.clone();
        }

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
            TokenType::And => Ok(Value::Boolean(
                self.test_truthness(&lhs) && self.test_truthness(&rhs),
            )),
            TokenType::Or => Ok(Value::Boolean(
                self.test_truthness(&lhs) || self.test_truthness(&rhs),
            )),
            _ => unreachable!(),
        }
    }

    fn evaluate_if_expression(
        &mut self,
        condition: Box<Expression>,
        true_branch: Box<Expression>,
        else_branch: Option<Box<Expression>>,
    ) -> Result<Value, RuntimeError> {
        let condition = self.evaluate_expression(*condition)?;

        if self.test_truthness(&condition) {
            return self.evaluate_expression(*true_branch);
        } else if let Some(else_branch) = else_branch {
            return self.evaluate_expression(*else_branch);
        }

        Ok(Value::Null)
    }

    fn evaluate_while_statement(
        &mut self,
        condition: Expression,
        block: Expression,
    ) -> Result<Value, RuntimeError> {
        self.environment.breakpoint = true;
        loop {
            let condition = self.evaluate_expression(condition.clone())?;
            if !self.test_truthness(&condition) {
                return Ok(Value::Null);
            }

            if let Err(err) = self.evaluate_expression(block.clone()) {
                if let RuntimeError::ControlFlow(statement) = err {
                    match statement {
                        ControlFlow::Break(_) => {
                            self.environment = self.environment.return_breakpoint();
                            break;
                        }
                        ControlFlow::Continue(_) => {
                            self.environment = self.environment.return_breakpoint();
                            continue;
                        }
                        ControlFlow::Return(value, token) => {
                            return Err(RuntimeError::ControlFlow(ControlFlow::Return(
                                value, token,
                            )))
                        }
                    }
                } else {
                    return Err(err);
                }
            };
        }

        Ok(Value::Null)
    }

    fn test_truthness(&self, value: &Value) -> bool {
        match value {
            Value::Number(value) => *value != 0.0,
            Value::Boolean(value) => *value,
            Value::Null => false,
            Value::String(value) => value.len() != 0,
        }
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use crate::{parser::parser::Parser, runtime::value::Value, tokenizer::lexer::Lexer};

    use super::Interpreter;

    #[test]
    fn test_break() {
        let s = "
            set i = 0;
            while (true) {
                if (true) {
                    if (true) {
                        break;
                    }
                }
                i = i + 1;
            }
        ";
        let tokens = Lexer::new(s).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let mut interpreter = Interpreter::new();

        for node in ast {
            interpreter.evaluate_statement(node);
        }

        let i_value = interpreter.environment.get("i".to_string()).unwrap();

        assert_eq!(i_value, Value::Number(0.0));
    }

    #[test]
    fn test_continue() {
        let s = "
            set i = 0;
            set j = 0;
            while i < 5 {
                i = i + 1;
                if i == 1 {
                    continue;
                }
                j = j + 1;
            }
        ";
        let tokens = Lexer::new(s).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let mut interpreter = Interpreter::new();

        for node in ast {
            interpreter.evaluate_statement(node);
        }

        let i_value = interpreter.environment.get("i".to_string()).unwrap();
        let j_value = interpreter.environment.get("j".to_string()).unwrap();

        assert_eq!(i_value, Value::Number(5.0));
        assert_eq!(j_value, Value::Number(4.0));
    }
}
