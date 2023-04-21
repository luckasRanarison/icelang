use crate::{
    environment::RefEnv,
    error::RuntimeError,
    value::{Builtin, Value},
    EvalExpr, Interpreter,
};
use lexer::tokens::Token;
use parser::ast::Expression;
use std::{f64::INFINITY, fs::read_to_string};

pub fn get_builtins() -> Vec<Builtin> {
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
    ]
}
