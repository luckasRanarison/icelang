pub mod builtin;
pub mod environment;
pub mod error;
pub mod value;

use crate::builtin::Builtin;

use environment::{Environment, RefEnv};
use error::{ControlFlow, RuntimeError};
use lexer::{tokens::TokenType, Lexer};
use parser::{ast::*, Parser};
use value::{Function, Range, RefVal, Value};

use std::{cell::RefCell, collections::HashMap, f64::INFINITY, ops, path::PathBuf, rc::Rc};

pub struct Interpreter {
    environment: RefEnv,
}

impl Interpreter {
    pub fn new(path: PathBuf) -> Self {
        let environment = Rc::new(RefCell::new(Environment::new(path)));

        Self { environment }
    }

    pub fn load_builtin(&self, builtins: Vec<Builtin>) {
        for builtin in builtins {
            self.environment
                .borrow_mut()
                .set(builtin.name, Value::Builtin(builtin));
        }
    }

    pub fn interpret<T: Eval>(&self, node: T) -> Result<Option<Value>, RuntimeError> {
        Ok(node.evaluate(&self.environment)?)
    }

    pub fn run_source(&self, source: &str) -> Result<Value, RuntimeError> {
        let tokens = match Lexer::new(source).tokenize() {
            Ok(value) => value,
            Err(error) => return Err(RuntimeError::LexicalError(error)),
        };
        let nodes = match Parser::new(&tokens).parse() {
            Ok(value) => value,
            Err(error) => return Err(RuntimeError::ParsingError(error)),
        };

        for node in nodes {
            if let Err(error) = self.interpret(node) {
                if let RuntimeError::Export(value) = error {
                    return Ok(value);
                } else {
                    return Err(error);
                }
            }
        }

        Ok(Value::Null)
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Number(value) => *value != 0.0,
        Value::Boolean(value) => *value,
        Value::Null => false,
        Value::String(value) => value.len() != 0,
        Value::Array(value) => !value.is_empty(),
        Value::Object(value) => value.values.len() != 0,
        _ => true,
    }
}

fn get_numerical_index(expr: &Index, value: Value) -> Result<usize, RuntimeError> {
    if let Value::Number(index) = value {
        if index < 0.0 {
            return Err(RuntimeError::InvalidIndex(expr.token.clone()));
        }
        Ok(index as usize)
    } else {
        Err(RuntimeError::InvalidIndex(expr.token.clone()))
    }
}

trait EvalRef {
    fn evaluate_ref(&self, env: &RefEnv) -> Result<RefVal, RuntimeError>;
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

trait EvalStmt {
    fn evaluate_statement(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError>;
}

impl EvalStmt for Statement {
    fn evaluate_statement(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        match self {
            Statement::ExpressionStatement(stmt) => stmt.evaluate(env),
            Statement::VariableDeclaration(stmt) => stmt.evaluate(env),
            Statement::BlockStatement(stmt) => stmt.evaluate(env),
            Statement::WhileStatement(stmt) => stmt.evaluate(env),
            Statement::LoopStatement(stmt) => stmt.evaluate(env),
            Statement::BreakStatement(stmt) => stmt.evaluate(env),
            Statement::ContinueStatement(stmt) => stmt.evaluate(env),
            Statement::FunctionDeclaration(stmt) => stmt.evaluate(env),
            Statement::ReturnStatement(stmt) => stmt.evaluate(env),
            Statement::ForStatement(stmt) => stmt.evaluate(env),
        }
    }
}

impl Eval for Declaration {
    fn evaluate(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        let name = &self.name.lexeme;

        if env.borrow().contains(name) {
            return Err(RuntimeError::RedeclaringIdentifier(self.name.clone()));
        }

        let value = self.value.evaluate_expression(env)?;
        env.borrow_mut().set(&name, value);

        Ok(None)
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

impl Eval for For {
    fn evaluate(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        let value = self.iterable.evaluate_expression(env)?;

        if !value.is_iterable() {
            return Err(RuntimeError::NonIterable(self.iterable_token.clone()));
        }

        let new_env = Rc::new(RefCell::new(Environment::from(env.clone())));

        for (key, value) in value.iter() {
            if let Some(second) = &self.variable.1 {
                new_env.borrow_mut().set(&self.variable.0.lexeme, key);
                new_env.borrow_mut().set(&second.lexeme.clone(), value);
            } else {
                new_env.borrow_mut().set(&self.variable.0.lexeme, value);
            }

            self.block.evaluate(&new_env)?;
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
                            return Err(RuntimeError::ControlFlow(ControlFlow::Return(
                                value, token,
                            )))
                        }
                    },
                    _ => return Err(error),
                }
            }
        }

        Ok(None)
    }
}

impl Eval for Loop {
    fn evaluate(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        loop {
            if let Some(error) = self.block.evaluate(env).err() {
                match error {
                    RuntimeError::ControlFlow(statement) => match statement {
                        ControlFlow::Break(_) => break,
                        ControlFlow::Continue(_) => continue,
                        ControlFlow::Return(value, token) => {
                            return Err(RuntimeError::ControlFlow(ControlFlow::Return(
                                value, token,
                            )))
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
        Err(RuntimeError::ControlFlow(ControlFlow::Break(
            self.token.clone(),
        )))
    }
}

impl Eval for Continue {
    fn evaluate(&self, _env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        Err(RuntimeError::ControlFlow(ControlFlow::Continue(
            self.token.clone(),
        )))
    }
}

impl Eval for FunctionDeclaration {
    fn evaluate(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        let name = &self.token.as_ref().unwrap().lexeme;

        if env.borrow().contains(name) {
            return Err(RuntimeError::RedeclaringIdentifier(
                self.token.as_ref().unwrap().clone(),
            ));
        }

        env.borrow_mut().set(
            &name,
            Value::Function(Function {
                declaration: self.clone(),
            }),
        );

        Ok(None)
    }
}

impl Eval for Return {
    fn evaluate(&self, env: &RefEnv) -> Result<Option<Value>, RuntimeError> {
        let value = self.expression.evaluate_expression(env)?;
        Err(RuntimeError::ControlFlow(ControlFlow::Return(
            value,
            self.token.clone(),
        )))
    }
}

pub trait EvalExpr {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError>;
}

impl EvalExpr for Expression {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError> {
        match self {
            Expression::LiteralExpression(expr) => expr.evaluate_expression(env),
            Expression::AssignementExpression(expr) => expr.evaluate_expression(env),
            Expression::ArrayExpression(expr) => expr.evaluate_expression(env),
            Expression::IndexExpression(expr) => expr.evaluate_expression(env),
            Expression::VariableExpression(expr) => expr.evaluate_expression(env),
            Expression::UnaryExpression(expr) => expr.evaluate_expression(env),
            Expression::BinaryExpression(expr) => expr.evaluate_expression(env),
            Expression::IfExpression(expr) => expr.evaluate_expression(env),
            Expression::MatchExpression(expr) => expr.evaluate_expression(env),
            Expression::FunctionCall(expr) => expr.evaluate_expression(env),
            Expression::LambdaFunction(expr) => expr.evaluate_expression(env),
            Expression::ObjectExpression(expr) => expr.evaluate_expression(env),
            Expression::PropAccess(expr) => expr.evaluate_expression(env),
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

        if let Some(value) = env.borrow().get(name) {
            Ok(value)
        } else {
            Err(RuntimeError::UndefinedIdentifier(self.token.clone()))
        }
    }
}

impl EvalRef for Variable {
    fn evaluate_ref(&self, env: &RefEnv) -> Result<RefVal, RuntimeError> {
        let name = &self.token.lexeme;

        if let Some(value) = env.borrow().get_ref(name) {
            Ok(value)
        } else {
            Err(RuntimeError::UndefinedIdentifier(self.token.clone()))
        }
    }
}

impl EvalExpr for Assign {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError> {
        let expression_value = self.value.evaluate_expression(env)?;

        let rf = match &*self.left {
            Expression::VariableExpression(variable) => variable.evaluate_ref(env)?,
            Expression::IndexExpression(index_expr) => index_expr.evaluate_ref(env)?,
            Expression::PropAccess(prop) => prop.evaluate_ref(env)?,
            _ => unreachable!(),
        };

        if let TokenType::Equal = &self.token.value {
            *rf.borrow_mut() = expression_value.clone();
            Ok(expression_value)
        } else {
            let prev = rf.borrow().clone();
            let result = match self.token.value {
                TokenType::PlusEqual => prev + expression_value,
                TokenType::MinusEqaul => prev - expression_value,
                TokenType::AsterixEqual => prev * expression_value,
                TokenType::SlashEqual => prev / expression_value,
                TokenType::ModuloEqual => prev % expression_value,
                _ => unreachable!(),
            };
            if let Some(result) = result {
                *rf.borrow_mut() = result.clone();
                Ok(result)
            } else {
                Err(RuntimeError::InvalidAssignment(self.token.clone()))
            }
        }
    }
}

impl EvalExpr for Array {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError> {
        let mut array: Vec<RefVal> = vec![];

        for item in &self.items {
            let expression = item.evaluate_expression(env)?;
            let rf = Rc::new(RefCell::new(expression));
            array.push(rf);
        }

        Ok(Value::Array(array))
    }
}

impl EvalExpr for Object {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError> {
        let mut values: HashMap<String, RefVal> = HashMap::new();

        for (token, expression) in &self.props {
            let name = match &token.value {
                TokenType::String(value) => value,
                _ => &token.lexeme,
            };
            let rf = Rc::new(RefCell::new(expression.evaluate_expression(env)?));

            values.insert(name.to_owned(), rf);
        }

        Ok(Value::Object(self::value::Object { values }))
    }
}

impl EvalExpr for Index {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError> {
        let expression = self.expression.evaluate_expression(env)?;
        let index_expression = self.index.evaluate_expression(env)?;

        if let Value::Object(object) = expression {
            let index = match index_expression {
                Value::Number(value) => value.to_string(),
                Value::String(value) => value,
                _ => return Err(RuntimeError::InvalidIndex(self.token.clone())),
            };

            let value = if let Some(value) = object.values.get(&index) {
                value.borrow().clone()
            } else {
                Value::Null
            };

            Ok(value)
        } else {
            let index = get_numerical_index(&self, index_expression)?;
            let value = match expression {
                Value::Array(array) => {
                    if let Some(value) = array.get(index) {
                        value.borrow().clone()
                    } else {
                        Value::Null
                    }
                }
                Value::String(string) => {
                    if let Some(value) = string.chars().nth(index) {
                        Value::String(value.to_string())
                    } else {
                        Value::Null
                    }
                }
                _ => return Err(RuntimeError::UnindexableType(self.token.clone())),
            };

            Ok(value)
        }
    }
}

impl EvalRef for Index {
    fn evaluate_ref(&self, env: &RefEnv) -> Result<RefVal, RuntimeError> {
        let expression_ref = match &*self.expression {
            Expression::VariableExpression(variable) => variable.evaluate_ref(env)?,
            Expression::IndexExpression(index_expr) => index_expr.evaluate_ref(env)?,
            Expression::PropAccess(prop) => prop.evaluate_ref(env)?,
            _ => return Err(RuntimeError::InvalidAssignment(self.token.clone())),
        };
        let index_expression = self.index.evaluate_expression(env)?;
        let expression = &mut *expression_ref.borrow_mut();

        match expression {
            Value::Array(array) => {
                let index = get_numerical_index(&self, index_expression)?;
                if index >= array.len() {
                    array.resize_with(index + 1, || Rc::new(RefCell::new(Value::Null)))
                }

                Ok(array[index].clone())
            }
            Value::Object(object) => {
                let index = match index_expression {
                    Value::Number(value) => value.to_string(),
                    Value::String(value) => value,
                    _ => return Err(RuntimeError::InvalidIndex(self.token.clone())),
                };

                if let Some(value) = object.values.get(&index) {
                    Ok(value.clone())
                } else {
                    let rf = Rc::new(RefCell::new(Value::Null));
                    object.values.insert(index, rf.clone());
                    Ok(rf)
                }
            }
            _ => Err(RuntimeError::InvalidAssignment(self.token.clone())),
        }
    }
}

impl EvalExpr for Access {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError> {
        let expression = self.expression.evaluate_expression(env)?;
        let property = &self.prop.lexeme;

        let value = match expression {
            Value::Object(object) => {
                if let Some(value) = object.values.get(property) {
                    value.borrow().clone()
                } else {
                    Value::Null
                }
            }
            _ => return Err(RuntimeError::NotAnObject(self.token.clone())),
        };

        Ok(value)
    }
}

impl EvalRef for Access {
    fn evaluate_ref(&self, env: &RefEnv) -> Result<RefVal, RuntimeError> {
        let expression_ref = match &*self.expression {
            Expression::VariableExpression(variable) => variable.evaluate_ref(env)?,
            Expression::IndexExpression(index_expr) => index_expr.evaluate_ref(env)?,
            Expression::PropAccess(prop) => prop.evaluate_ref(env)?,
            _ => return Err(RuntimeError::InvalidAssignment(self.token.clone())),
        };
        let expression = &mut *expression_ref.borrow_mut();

        if let Value::Object(object) = expression {
            let prop = &self.prop.lexeme;
            if let Some(value) = object.values.get(prop) {
                Ok(value.clone())
            } else {
                let rf = Rc::new(RefCell::new(Value::Null));
                object.values.insert(prop.to_owned(), rf.clone());
                Ok(rf)
            }
        } else {
            Err(RuntimeError::NotAnObject(self.token.clone()))
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
            TokenType::Bang => Ok(Value::Boolean(!is_truthy(&operand))),
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
                    format!("cannot multiply a '{}' by a '{}'", left_type, right_type),
                    self.operator.pos,
                )),
            },
            TokenType::Slash => {
                if let Value::Number(right) = right {
                    if right == 0.0 {
                        return Err(RuntimeError::DivisionByZero(self.operator.pos));
                    }
                }

                match left / right {
                    Some(value) => Ok(value),
                    None => Err(RuntimeError::InvalidOperation(
                        format!("cannot divide a '{}' by a '{}'", left_type, right_type),
                        self.operator.pos,
                    )),
                }
            }
            TokenType::Minus => match left - right {
                Some(value) => Ok(value),
                None => Err(RuntimeError::InvalidOperation(
                    format!("cannot substract a '{}' by a '{}'", left_type, right_type),
                    self.operator.pos,
                )),
            },
            TokenType::Plus => match left + right {
                Some(value) => Ok(value),
                None => Err(RuntimeError::InvalidOperation(
                    format!("cannot add a '{}' by a '{}'", left_type, right_type),
                    self.operator.pos,
                )),
            },
            TokenType::Modulo => match left % right {
                Some(value) => Ok(value),
                None => Err(RuntimeError::InvalidOperation(
                    format!("cannot divide a '{}' by a '{}'", left_type, right_type),
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
            TokenType::To => match (left, right) {
                (Value::Number(start), Value::Number(end)) => {
                    let start = start as i64;
                    let end = end as i64;
                    let range = Range::NumberRange(ops::Range { start, end });
                    Ok(Value::Range(range))
                }
                (Value::String(start), Value::String(end)) => {
                    if start.len() == 1 && end.len() == 1 {
                        let start = start.chars().next().unwrap();
                        let end = end.chars().next().unwrap();
                        let range = Range::CharRange(ops::Range { start, end });
                        Ok(Value::Range(range))
                    } else {
                        Err(RuntimeError::InvalidRange(self.operator.clone()))
                    }
                }
                _ => Err(RuntimeError::InvalidRange(self.operator.clone())),
            },
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

impl EvalExpr for Match {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError> {
        let match_pattern = self.pattern.evaluate_expression(env)?;

        for arm in &self.arms {
            for pattern in &arm.pattern {
                let value = pattern.evaluate_expression(env)?;

                if match_pattern == value {
                    match &arm.block.evaluate(env)? {
                        Some(value) => return Ok(value.clone()),
                        None => return Ok(Value::Null),
                    }
                }
            }
        }

        if let Some(defalut) = &self.default {
            match defalut.block.evaluate(env)? {
                Some(value) => return Ok(value.clone()),
                None => return Ok(Value::Null),
            }
        }

        Ok(Value::Null)
    }
}

impl EvalExpr for Lambda {
    fn evaluate_expression(&self, _env: &RefEnv) -> Result<Value, RuntimeError> {
        let lambda = Value::Function(Function {
            declaration: FunctionDeclaration {
                token: None,
                parameter: self.parameter.clone(),
                body: self.body.clone(),
            },
        });

        Ok(lambda)
    }
}

impl EvalExpr for Call {
    fn evaluate_expression(&self, env: &RefEnv) -> Result<Value, RuntimeError> {
        let value = self.caller.evaluate_expression(env)?;
        let got = self.arguments.len();

        if let Value::Function(function) = value {
            let new_env = Rc::new(RefCell::new(Environment::from(env.clone())));
            let expected = function.declaration.parameter.len();

            if expected != got {
                return Err(RuntimeError::InvalidArgument(
                    expected,
                    got,
                    self.token.clone(),
                ));
            }

            if let Some(object) = &self.object {
                let object_value = object.evaluate_expression(env)?;
                new_env.borrow_mut().set("self", object_value);
            }

            for (index, arg) in self.arguments.iter().enumerate() {
                let arg_value = arg.evaluate_expression(env)?;
                let arg_name = &function.declaration.parameter[index].lexeme;
                new_env.borrow_mut().set(arg_name, arg_value);
            }

            let value = function.declaration.body.evaluate(&new_env);
            let value = match value {
                Ok(value) => value,
                Err(error) => {
                    if let RuntimeError::ControlFlow(ControlFlow::Return(value, _)) = error {
                        Some(value)
                    } else {
                        return Err(error);
                    }
                }
            };

            match value {
                Some(value) => Ok(value),
                None => Ok(Value::Null),
            }
        } else if let Value::Builtin(builtin) = value {
            let expected = builtin.args;

            if expected != INFINITY as usize && got != expected {
                return Err(RuntimeError::InvalidArgument(
                    expected,
                    got,
                    self.token.clone(),
                ));
            }

            let value = (builtin.function)(env, &self.token, &self.arguments)?;

            Ok(value)
        } else {
            return Err(RuntimeError::NotFunciton(self.token.clone()));
        }
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use crate::value::Object;

    use super::{Interpreter, Value};
    use lexer::Lexer;
    use parser::Parser;
    use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

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
        let interpreter = Interpreter::new(PathBuf::new());
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
        let interpreter = Interpreter::new(PathBuf::new());
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
        let interpreter = Interpreter::new(PathBuf::new());
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
        let interpreter = Interpreter::new(PathBuf::new());
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();

        assert_eq!(get("i"), Value::Number(5.0));
    }

    #[test]
    fn test_loop() {
        let source = "
            set i = 0;
            loop {
                if (i == 5) {
                    break;
                }
                i = i + 1;
            } 
        ";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let interpreter = Interpreter::new(PathBuf::new());
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
        let interpreter = Interpreter::new(PathBuf::new());
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();

        assert_eq!(get("i"), Value::Number(6.0));
    }

    #[test]

    fn test_match() {
        let source = "
            set a = 3;
            set b = match a {
                1: false,
                false: {
                    'unreachable'
                },
                6, 3: {
                    true
                },
            };
        ";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let interpreter = Interpreter::new(PathBuf::new());
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();

        assert_eq!(get("b"), Value::Boolean(true));
    }

    #[test]
    fn test_array() {
        let source = "
            set a = [1, 'hi', true];
            set b = a[0];
            set c = a[1];
            set d = a[2];
            set e = a[3];
        ";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let interpreter = Interpreter::new(PathBuf::new());
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();

        assert_eq!(get("b"), Value::Number(1.0));
        assert_eq!(get("c"), Value::String("hi".to_owned()));
        assert_eq!(get("d"), Value::Boolean(true));
        assert_eq!(get("e"), Value::Null);
    }

    #[test]
    fn test_index_assignment() {
        let source = "
            set a = [1, 2, [0, 1]];
            a[1] = a[0];
            a[0] = 0;
            a[2][2] = 2;
            a[4] = 3;
        ";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let interpreter = Interpreter::new(PathBuf::new());
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();

        assert_eq!(
            get("a"),
            Value::Array(vec![
                Rc::new(RefCell::new(Value::Number(0.0))),
                Rc::new(RefCell::new(Value::Number(1.0))),
                Rc::new(RefCell::new(Value::Array(vec![
                    Rc::new(RefCell::new(Value::Number(0.0))),
                    Rc::new(RefCell::new(Value::Number(1.0))),
                    Rc::new(RefCell::new(Value::Number(2.0))),
                ]))),
                Rc::new(RefCell::new(Value::Null)),
                Rc::new(RefCell::new(Value::Number(3.0))),
            ])
        );
    }

    #[test]
    fn test_function() {
        let source = "
            function hello() {
                'Hello World'
            }
        
            set a  = hello();

            function salute(name) {
                return 'Hello ' + name;
            }

            set b = salute('Luckas');
            
            function countdown(n) {
                if n > 0 {
                    return countdown(n - 1);
                }

                return n;
            }
            
            set c = countdown(5);
        ";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let interpreter = Interpreter::new(PathBuf::new());
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();

        assert_eq!(get("a"), Value::String("Hello World".to_owned()));
        assert_eq!(get("b"), Value::String("Hello Luckas".to_owned()));
        assert_eq!(get("c"), Value::Number(0.0));
    }

    #[test]
    fn test_lambda() {
        let source = "
            set a = (lambda() 'hi')()
            set b = lambda() {
                [1, lambda() { 'here' }, 3, 4]
            }()[1]()
        ";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let interpreter = Interpreter::new(PathBuf::new());
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();

        assert_eq!(get("a"), Value::String("hi".to_owned()));
        assert_eq!(get("b"), Value::String("here".to_owned()));
    }

    #[test]
    fn test_object() {
        let source = "
            set me = {
                name: 'Luckas',
                age: 17,
                salute: lambda(other) 'Hello ' + other.name + ' I\\'m ' + self.name
            };
            set name = me.name;
            set age = me['age'];
            set other = { name: 'stranger' };
            set salute = me.salute(other);
            me.happy = true;
            set emotion = me.happy;
        ";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let interpreter = Interpreter::new(PathBuf::new());
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();

        assert_eq!(get("name"), Value::String("Luckas".to_owned()));
        assert_eq!(get("age"), Value::Number(17.0));
        assert_eq!(
            get("salute"),
            Value::String("Hello stranger I'm Luckas".to_owned())
        );
        assert_eq!(get("emotion"), Value::Boolean(true));
    }

    #[test]
    fn test_for() {
        let source = "
            set a = ['among', 'us'];
            set o = {};
            for key, value in a {
                o[value] = key
            }
        ";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let interpreter = Interpreter::new(PathBuf::new());
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();

        let mut object = Object {
            values: HashMap::new(),
        };

        object.values.insert(
            "among".to_owned(),
            Rc::new(RefCell::new(Value::Number(0.0))),
        );
        object
            .values
            .insert("us".to_owned(), Rc::new(RefCell::new(Value::Number(1.0))));

        assert_eq!(get("o"), Value::Object(object));
    }

    #[test]
    fn test_range() {
        let source = "
            set factorial = 1;
            for i in 1 to 6 {
                factorial *= i;
            }
        ";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let interpreter = Interpreter::new(PathBuf::new());
        for node in ast {
            interpreter.interpret(node);
        }
        let get = |name| interpreter.environment.as_ref().borrow().get(name).unwrap();

        assert_eq!(get("factorial"), Value::Number(120.0));
    }
}
