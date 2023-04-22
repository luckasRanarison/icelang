use crate::{
    environment::RefEnv,
    error::RuntimeError,
    value::{Builtin, Value},
    EvalExpr, Interpreter,
};
use lexer::tokens::Token;
use parser::ast::Expression;
use std::{f64::INFINITY, fs::read_to_string, io};

pub fn get_std_builtins() -> Vec<Builtin> {
    vec![
        Builtin {
            name: "type_of",
            args: 1,
            function: |env: &RefEnv,
                       _: &Token,
                       args: &Vec<Expression>|
             -> Result<Value, RuntimeError> {
                let value = &args[0];
                let value = value.evaluate_expression(env)?;
                let value_type = Value::String(value.get_type());

                Ok(value_type)
            },
        },
        Builtin {
            name: "length",
            args: 1,
            function: |env: &RefEnv,
                       _: &Token,
                       args: &Vec<Expression>|
             -> Result<Value, RuntimeError> {
                let arg = &args[0];
                let value = arg.evaluate_expression(env)?;
                let value = match value {
                    Value::String(string) => Value::Number(string.len() as f64),
                    Value::Array(array) => Value::Number(array.len() as f64),
                    Value::Object(object) => Value::Number(object.values.len() as f64),
                    _ => Value::Null,
                };

                Ok(value)
            },
        },
        Builtin {
            name: "sqrt",
            args: 1,
            function: |env: &RefEnv,
                       token: &Token,
                       args: &Vec<Expression>|
             -> Result<Value, RuntimeError> {
                let arg = &args[0];
                let value = arg.evaluate_expression(env)?;
                let value = match value {
                    Value::Number(value) => Value::Number(value.sqrt()),
                    _ => {
                        return Err(RuntimeError::ExpectedButGot(
                            "number".to_owned(),
                            value.get_type(),
                            token.clone(),
                        ))
                    }
                };

                Ok(value)
            },
        },
        Builtin {
            name: "pow",
            args: 2,
            function: |env: &RefEnv,
                       token: &Token,
                       args: &Vec<Expression>|
             -> Result<Value, RuntimeError> {
                let value = &args[0];
                let exponent = &args[1];
                let value = value.evaluate_expression(env)?;
                let exponent = exponent.evaluate_expression(env)?;
                let value = match (value, exponent) {
                    (Value::Number(value), Value::Number(exponent)) => {
                        Value::Number(value.powf(exponent))
                    }
                    _ => return Err(RuntimeError::MismatchedArg(token.clone())),
                };

                Ok(value)
            },
        },
    ]
}

pub fn get_io_builtins() -> Vec<Builtin> {
    vec![
        Builtin {
            name: "print",
            args: INFINITY as usize,
            function: |env: &RefEnv,
                       _: &Token,
                       args: &Vec<Expression>|
             -> Result<Value, RuntimeError> {
                for arg in args {
                    let value = arg.evaluate_expression(env)?;
                    print!("{value}");
                }

                println!("");
                Ok(Value::Null)
            },
        },
        Builtin {
            name: "readline",
            args: 0,
            function: |_: &RefEnv, _: &Token, _: &Vec<Expression>| -> Result<Value, RuntimeError> {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");

                Ok(Value::String(input.trim_end().to_string()))
            },
        },
        Builtin {
            name: "import",
            args: 1,
            function: |env: &RefEnv,
                       token: &Token,
                       args: &Vec<Expression>|
             -> Result<Value, RuntimeError> {
                let arg = &args[0];
                let mut value = arg.evaluate_expression(env)?;
                let file_path = if let Value::String(value) = &mut value {
                    if value.ends_with(".ic") {
                        value
                    } else {
                        value.push_str(".ic");
                        value
                    }
                } else {
                    return Err(RuntimeError::InvalidPath(value, token.clone()));
                };
                let source = match read_to_string(&file_path) {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(RuntimeError::ModuleNotFound(
                            file_path.to_owned(),
                            token.clone(),
                        ))
                    }
                };
                let value = Interpreter::run_source(&source)?;

                Ok(value)
            },
        },
        Builtin {
            name: "export",
            args: 1,
            function: |env: &RefEnv,
                       _: &Token,
                       args: &Vec<Expression>|
             -> Result<Value, RuntimeError> {
                let arg = &args[0];
                let value = arg.evaluate_expression(env)?;

                Err(RuntimeError::Export(value))
            },
        },
    ]
}
