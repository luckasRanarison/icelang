use crate::{environment::RefEnv, error::RuntimeError, value::Value, EvalExpr, Interpreter};

use lexer::tokens::Token;
use parser::ast::Expression;
use std::{f64::INFINITY, fmt, fs::read_to_string, io};

type BuiltinFn = fn(&RefEnv, token: &Token, &Vec<Expression>) -> Result<Value, RuntimeError>;

#[derive(Clone)]
pub struct Builtin {
    pub name: &'static str,
    pub args: usize,
    pub function: BuiltinFn,
}

impl Builtin {
    pub fn new(name: &'static str, args: usize, function: BuiltinFn) -> Self {
        Self {
            name,
            args,
            function,
        }
    }
}

impl fmt::Debug for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Builtin")
            .field("name", &self.name)
            .field("args", &self.args)
            .field("function", &"<native function>")
            .finish()
    }
}

impl PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for Builtin {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Less)
    }
}

pub fn get_std_builtins() -> Vec<Builtin> {
    vec![
        Builtin::new("type_of", 1, type_of),
        Builtin::new("length", 1, length),
        Builtin::new("sqrt", 1, sqrt),
        Builtin::new("pow", 2, pow),
        Builtin::new("floor", 1, floor),
        Builtin::new("round", 1, round),
        Builtin::new("ceil", 1, ceil),
        Builtin::new("parse_number", 1, parse_number),
    ]
}

pub fn get_io_builtins() -> Vec<Builtin> {
    vec![
        Builtin::new("print", INFINITY as usize, io_print),
        Builtin::new("readline", 0, io_readline),
        Builtin::new("import", 1, import),
        Builtin::new("export", 1, export),
    ]
}

fn type_of(env: &RefEnv, _: &Token, args: &Vec<Expression>) -> Result<Value, RuntimeError> {
    let value = &args[0];
    let value = value.evaluate_expression(env)?;
    let value_type = Value::String(value.get_type());

    Ok(value_type)
}

fn length(env: &RefEnv, token: &Token, args: &Vec<Expression>) -> Result<Value, RuntimeError> {
    let arg = &args[0];
    let value = arg.evaluate_expression(env)?;

    match value {
        Value::String(string) => Ok(Value::Number(string.len() as f64)),
        Value::Array(array) => Ok(Value::Number(array.len() as f64)),
        Value::Object(object) => Ok(Value::Number(object.values.len() as f64)),
        _ => Err(RuntimeError::InvalidArg(token.clone())),
    }
}

fn sqrt(env: &RefEnv, token: &Token, args: &Vec<Expression>) -> Result<Value, RuntimeError> {
    let arg = &args[0];
    let value = arg.evaluate_expression(env)?;

    match value {
        Value::Number(value) => Ok(Value::Number(value.sqrt())),
        _ => Err(RuntimeError::ExpectedButGot(
            "number".to_owned(),
            value.get_type(),
            token.clone(),
        )),
    }
}

fn pow(env: &RefEnv, token: &Token, args: &Vec<Expression>) -> Result<Value, RuntimeError> {
    let value = &args[0];
    let exponent = &args[1];
    let value = value.evaluate_expression(env)?;
    let exponent = exponent.evaluate_expression(env)?;

    match (value, exponent) {
        (Value::Number(value), Value::Number(exponent)) => Ok(Value::Number(value.powf(exponent))),
        _ => Err(RuntimeError::MismatchedArg(token.clone())),
    }
}

fn floor(env: &RefEnv, token: &Token, args: &Vec<Expression>) -> Result<Value, RuntimeError> {
    let arg = &args[0];
    let value = arg.evaluate_expression(env)?;

    match value {
        Value::Number(value) => Ok(Value::Number(value.floor())),
        _ => Err(RuntimeError::ExpectedButGot(
            "number".to_owned(),
            value.get_type(),
            token.clone(),
        )),
    }
}

fn round(env: &RefEnv, token: &Token, args: &Vec<Expression>) -> Result<Value, RuntimeError> {
    let arg = &args[0];
    let value = arg.evaluate_expression(env)?;

    match value {
        Value::Number(value) => Ok(Value::Number(value.round())),
        _ => {
            return Err(RuntimeError::ExpectedButGot(
                "number".to_owned(),
                value.get_type(),
                token.clone(),
            ))
        }
    }
}

fn ceil(env: &RefEnv, token: &Token, args: &Vec<Expression>) -> Result<Value, RuntimeError> {
    let arg = &args[0];
    let value = arg.evaluate_expression(env)?;

    match value {
        Value::Number(value) => Ok(Value::Number(value.ceil())),
        _ => {
            return Err(RuntimeError::ExpectedButGot(
                "number".to_owned(),
                value.get_type(),
                token.clone(),
            ))
        }
    }
}

fn parse_number(
    env: &RefEnv,
    token: &Token,
    args: &Vec<Expression>,
) -> Result<Value, RuntimeError> {
    let arg = &args[0];
    let value = arg.evaluate_expression(env)?;

    match value {
        Value::String(value) => match value.parse::<f64>() {
            Ok(number) => Ok(Value::Number(number)),
            Err(_) => Err(RuntimeError::InvalidNumber(token.clone())),
        },
        _ => Err(RuntimeError::ExpectedButGot(
            "string".to_owned(),
            value.get_type(),
            token.clone(),
        )),
    }
}

fn io_print(env: &RefEnv, _: &Token, args: &Vec<Expression>) -> Result<Value, RuntimeError> {
    for arg in args {
        let value = arg.evaluate_expression(env)?;
        print!("{value}");
    }

    println!("");
    Ok(Value::Null)
}

fn io_readline(_: &RefEnv, _: &Token, _: &Vec<Expression>) -> Result<Value, RuntimeError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    Ok(Value::String(input.trim_end().to_string()))
}

fn import(env: &RefEnv, token: &Token, args: &Vec<Expression>) -> Result<Value, RuntimeError> {
    let arg = &args[0];
    let mut value = arg.evaluate_expression(env)?;
    let file_path = match &mut value {
        Value::String(value) => {
            if !value.ends_with(".ic") {
                value.push_str(".ic");
            }

            value
        }
        _ => return Err(RuntimeError::InvalidPath(value, token.clone())),
    };
    let file_path = &env.borrow().get_path().join(file_path);
    let source = match read_to_string(&file_path) {
        Ok(value) => value,
        Err(_) => {
            return Err(RuntimeError::ModuleNotFound(
                format!("{:?}", file_path),
                token.clone(),
            ))
        }
    };
    let path = file_path.parent().unwrap().to_path_buf();
    let interpreter = Interpreter::new(path);
    interpreter.load_builtin(get_std_builtins());
    interpreter.load_builtin(get_io_builtins());
    let value = interpreter.run_source(&source)?;

    Ok(value)
}

fn export(env: &RefEnv, _: &Token, args: &Vec<Expression>) -> Result<Value, RuntimeError> {
    let arg = &args[0];
    let value = arg.evaluate_expression(env)?;

    Err(RuntimeError::Export(value))
}
