use std::{cell::RefCell, rc::Rc};

use super::{
    environment::{Environment, RefEnv},
    error::{ControlFlow, RuntimeError},
    value::Value,
};
use crate::{
    lexer::tokens::TokenType,
    parser::ast::{
        Assignement, Binary, Block, Break, Continue, Declaration, Expression, If, Literal,
        Statement, Unary, Variable, While,
    },
};

pub struct Interpreter {
    environment: RefEnv,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret<T: Eval>(&self, node: T) -> Result<Option<Value>, RuntimeError> {
        Ok(node.evaluate(&self.environment)?)
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Number(value) => *value != 0.0,
        Value::Boolean(value) => *value,
        Value::Null => false,
        Value::String(value) => value.len() != 0,
    }
}

pub trait Eval {
    fn evaluate(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError>;
}

impl Eval for Statement {
    fn evaluate(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        self.evaluate_statement(env)
    }
}

impl Eval for Expression {
    fn evaluate(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        Ok(Some(self.evaluate_expression(env)?))
    }
}

pub trait EvalStmt {
    fn evaluate_statement(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError>;
}

impl EvalStmt for Statement {
    fn evaluate_statement(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        match self {
            Statement::ExpressionStatement(stmt) => stmt.evaluate(env),
            Statement::VariableDeclaration(stmt) => stmt.evaluate(env),
            Statement::VariableAssignement(stmt) => stmt.evaluate(env),
            Statement::BlockStatement(stmt) => stmt.evaluate(env),
            Statement::WhileStatement(stmt) => stmt.evaluate(env),
            Statement::BreakStatement(stmt) => stmt.evaluate(env),
            Statement::ContinueStatement(stmt) => stmt.evaluate(env),
        }
    }
}

impl Eval for Declaration {
    fn evaluate(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        let name = &self.name.lexeme;

        if env.borrow().contains(name) {
            return Err(RuntimeError::RedeclaringVariable(self.name.clone()));
        }

        let value = self.value.evaluate_expression(env)?;
        env.borrow_mut().set(&name, value);

        Ok(None)
    }
}

impl Eval for Assignement {
    fn evaluate(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        let name = &self.name.lexeme;
        let value = self.value.evaluate_expression(env)?;

        if env.borrow_mut().assign(name, value) {
            Ok(None)
        } else {
            Err(RuntimeError::UndefinedVariable(self.name.clone()))
        }
    }
}

impl Eval for Block {
    fn evaluate(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        let new_env = Rc::new(RefCell::new(Environment::from(env.clone())));
        let len = self.statements.len();

        for (index, statement) in self.statements.iter().enumerate() {
            if index == len - 1 {
                return statement.evaluate(&new_env);
            }
            statement.evaluate(&new_env)?;
        }

        Ok(None)
    }
}

impl Eval for While {
    fn evaluate(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        loop {
            let condition = self.condition.evaluate_expression(env)?;
            if !is_truthy(&condition) {
                break;
            }

            if let Some(error) = self.block.evaluate(env).err() {
                match error {
                    RuntimeError::ControlFlow(statement) => match statement {
                        ControlFlow::Break(_) => break,
                        ControlFlow::Continue(_) => continue,
                        ControlFlow::Return(value, token) => {
                            return Err(RuntimeError::ControlFlow(
                                super::error::ControlFlow::Return(value, token),
                            ))
                        }
                    },
                    _ => return Err(error),
                }
            }
        }

        Ok(None)
    }
}

impl Eval for Break {
    fn evaluate(&self, _env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        Err(RuntimeError::ControlFlow(super::error::ControlFlow::Break(
            self.token.clone(),
        )))
    }
}

impl Eval for Continue {
    fn evaluate(&self, _env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        Err(RuntimeError::ControlFlow(
            super::error::ControlFlow::Continue(self.token.clone()),
        ))
    }
}

pub trait EvalExpr {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError>;
}

impl EvalExpr for Expression {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError> {
        match self {
            Expression::LiteralExpression(expr) => expr.evaluate_expression(env),
            Expression::VariableExpression(expr) => expr.evaluate_expression(env),
            Expression::UnaryExpression(expr) => expr.evaluate_expression(env),
            Expression::BinaryExpression(expr) => expr.evaluate_expression(env),
            Expression::IfExpression(expr) => expr.evaluate_expression(env),
        }
    }
}

impl EvalExpr for Literal {
    fn evaluate_expression(&self, _env: &RefEnv) -> Result<Value, RuntimeError> {
        let value = match &self.token.value {
            TokenType::Number(value) => Value::Number(*value),
            TokenType::String(value) => Value::String(value.clone()),
            TokenType::Null => Value::Null,
            TokenType::True => Value::Boolean(true),
            TokenType::False => Value::Boolean(false),
            _ => unreachable!(),
        };

        Ok(value)
    }
}

impl EvalExpr for Variable {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError> {
        let name = &self.token.lexeme;

        if let Some(value) = env.borrow_mut().get(name) {
            Ok(value)
        } else {
            Err(RuntimeError::UndefinedVariable(self.token.clone()))
        }
    }
}

impl EvalExpr for Unary {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError> {
        let operand = self.operand.evaluate_expression(env)?;
        match &self.operator.value {
            TokenType::Minus => match operand {
                Value::Number(value) => Ok(Value::Number(-value)),
                _ => Err(RuntimeError::TypeExpection(
                    "number".to_owned(),
                    operand.get_type(),
                    self.operator.pos,
                )),
            },
            TokenType::Bang => match operand {
                Value::Boolean(value) => Ok(Value::Boolean(!value)),
                _ => Ok(Value::Boolean(true)),
            },
            _ => unreachable!(),
        }
    }
}

impl EvalExpr for Binary {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError> {
        let left = self.left.evaluate_expression(env)?;
        let right = self.right.evaluate_expression(env)?;
        let left_type = left.get_type();
        let right_type = right.get_type();

        match &self.operator.value {
            TokenType::Asterix => match left * right {
                Some(value) => Ok(value),
                None => Err(RuntimeError::InvalidOperation(
                    format!("can't multiply a '{}' by a '{}'", left_type, right_type),
                    self.operator.pos,
                )),
            },
            TokenType::Slash => match left / right {
                Some(value) => Ok(value),
                None => Err(RuntimeError::InvalidOperation(
                    format!("can't divide a '{}' by a '{}'", left_type, right_type),
                    self.operator.pos,
                )),
            },
            TokenType::Minus => match left - right {
                Some(value) => Ok(value),
                None => Err(RuntimeError::InvalidOperation(
                    format!("can't substract a '{}' by a '{}'", left_type, right_type),
                    self.operator.pos,
                )),
            },
            TokenType::Plus => match left + right {
                Some(value) => Ok(value),
                None => Err(RuntimeError::InvalidOperation(
                    format!("can't add a '{}' by a '{}'", left_type, right_type),
                    self.operator.pos,
                )),
            },
            TokenType::Modulo => match left % right {
                Some(value) => Ok(value),
                None => Err(RuntimeError::InvalidOperation(
                    format!("can't divide a '{}' by a '{}'", left_type, right_type),
                    self.operator.pos,
                )),
            },
            TokenType::Greater => Ok(Value::Boolean(left > right)),
            TokenType::GreaterEqual => Ok(Value::Boolean(left >= right)),
            TokenType::Less => Ok(Value::Boolean(left < right)),
            TokenType::LessEqual => Ok(Value::Boolean(left <= right)),
            TokenType::EqualEqual => Ok(Value::Boolean(left == right)),
            TokenType::BangEqual => Ok(Value::Boolean(left != right)),
            TokenType::And => Ok(Value::Boolean(is_truthy(&left) && is_truthy(&right))),
            TokenType::Or => Ok(Value::Boolean(is_truthy(&left) || is_truthy(&right))),
            _ => unreachable!(),
        }
    }
}

impl EvalExpr for If {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError> {
        let condition = self.condition.evaluate_expression(env)?;
        let mut value = None;

        if is_truthy(&condition) {
            value = self.true_branch.evaluate(env)?;
        } else if let Some(else_branch) = &self.else_branch {
            value = else_branch.evaluate(env)?;
        }

        match value {
            Some(value) => Ok(value),
            None => Ok(Value::Null),
        }
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use super::Interpreter;
    use crate::{lexer::Lexer, parser::Parser, runtime::value::Value};

    #[test]
    fn test_eval_operations() {
        let source = "
            set a = 2;
            set b = 4;
            set c = (a + b) % 2 == 0;
            b = a * b;
        ";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let interpreter = Interpreter::new();
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();

        assert_eq!(get("a"), Value::Number(2.0));
        assert_eq!(get("b"), Value::Number(8.0));
        assert_eq!(get("c"), Value::Boolean(true));
    }

    #[test]
    fn test_eval_block() {
        let source = "
            set a = 2;
            {
                a = 3;
                {
                    set a = 4;
                }
                set b = false;
            }
        ";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let interpreter = Interpreter::new();
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();
        let contains = |name| interpreter.environment.as_ref().borrow().contains(name);

        assert_eq!(get("a"), Value::Number(3.0));
        assert_eq!(contains("b"), false);
    }

    #[test]
    fn test_eval_if() {
        let source = "
            set a = true;
            set b = if a {1} else {0};
            a = 2;
            set c = 3;
            set max = if (a > b and a > c) {
                a
            } else if (b > a and b > c) {
                b
            } else {
                c
            };
        ";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let interpreter = Interpreter::new();
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();

        assert_eq!(get("a"), Value::Number(2.0));
        assert_eq!(get("b"), Value::Number(1.0));
        assert_eq!(get("max"), get("c"));
    }

    #[test]
    fn test_while() {
        let source = "
            set i = 0;
            while i < 5 {
                i = i + 1;
            }
        ";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let interpreter = Interpreter::new();
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();

        assert_eq!(get("i"), Value::Number(5.0));
    }

    #[test]
    fn test_control_flows() {
        let source = "
            set i = 0;
            while true {
                i = i + 1;
                if i == 3 {
                    continue;
                } 
                if (i % 3) == 0 {
                    break;
                }
            }
        ";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let interpreter = Interpreter::new();
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();

        assert_eq!(get("i"), Value::Number(6.0));
    }
}
