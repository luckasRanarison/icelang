use super::{environment::Environment, error::RuntimeError, value::Value};
use crate::{
    lexer::tokens::TokenType,
    parser::ast::{Assignement, Declaration, Expression, Literal, Statement, Unary, Variable},
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

    pub fn interpret<T: Eval>(&mut self, node: T) -> Result<Option<Value>, RuntimeError> {
        Ok(node.evaluate(&mut self.environment)?)
    }
}

pub trait Eval {
    fn evaluate(&self, env: &mut Environment) -> Result<Option<Value>, RuntimeError>;
}

impl Eval for Statement {
    fn evaluate(&self, env: &mut Environment) -> Result<Option<Value>, RuntimeError> {
        self.evaluate_statement(env)
    }
}

impl Eval for Expression {
    fn evaluate(&self, env: &mut Environment) -> Result<Option<Value>, RuntimeError> {
        Ok(Some(self.evaluate_expression(env)?))
    }
}

pub trait EvalStmt {
    fn evaluate_statement(&self, env: &mut Environment) -> Result<Option<Value>, RuntimeError>;
}

impl EvalStmt for Statement {
    fn evaluate_statement(&self, env: &mut Environment) -> Result<Option<Value>, RuntimeError> {
        match self {
            Statement::ExpressionStatement(stmt) => stmt.evaluate(env),
            Statement::VariableDeclaration(stmt) => stmt.evaluate(env),
            Statement::VariableAssignement(stmt) => stmt.evaluate(env),
            Statement::BlockStatement(_) => todo!(),
            Statement::WhileStatement(_) => todo!(),
            Statement::BreakStatement(_) => todo!(),
            Statement::ContinueStatement(_) => todo!(),
        }
    }
}

impl Eval for Declaration {
    fn evaluate(&self, env: &mut Environment) -> Result<Option<Value>, RuntimeError> {
        let name = &self.name.lexeme;
        if env.local_contains(name) {
            return Err(RuntimeError::RedeclaringVariable(self.name.clone()));
        }
        let value = self.value.evaluate_expression(env)?;
        env.set(&name, value);

        Ok(None)
    }
}

impl Eval for Assignement {
    fn evaluate(&self, env: &mut Environment) -> Result<Option<Value>, RuntimeError> {
        let name = &self.name.lexeme;
        let value = self.value.evaluate_expression(env)?;
        if env.assign(name, value) {
            Ok(None)
        } else {
            Err(RuntimeError::UndefinedVariable(self.name.clone()))
        }
    }
}

pub trait EvalExpr {
    fn evaluate_expression(&self, env: &mut Environment) -> Result<Value, RuntimeError>;
}

impl EvalExpr for Expression {
    fn evaluate_expression(&self, env: &mut Environment) -> Result<Value, RuntimeError> {
        match self {
            Expression::LiteralExpression(expr) => expr.evaluate_expression(env),
            Expression::VariableExpression(expr) => expr.evaluate_expression(env),
            Expression::UnaryExpression(_) => todo!(),
            Expression::BinaryExpression(_) => todo!(),
            Expression::IfExpression(_) => todo!(),
        }
    }
}

impl EvalExpr for Literal {
    fn evaluate_expression(&self, _env: &mut Environment) -> Result<Value, RuntimeError> {
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
    fn evaluate_expression(&self, env: &mut Environment) -> Result<Value, RuntimeError> {
        let name = &self.token.lexeme;
        if let Some(value) = env.get(name) {
            Ok(value)
        } else {
            Err(RuntimeError::UndefinedVariable(self.token.clone()))
        }
    }
}

impl EvalExpr for Unary {
    fn evaluate_expression(&self, env: &mut Environment) -> Result<Value, RuntimeError> {
        todo!()
    }
}
