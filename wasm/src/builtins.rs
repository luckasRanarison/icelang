use std::f64::INFINITY;

use interpreter::{
    environment::RefEnv,
    error::RuntimeError,
    value::{Builtin, Value},
    EvalExpr,
};
use lexer::tokens::Token;
use parser::ast::Expression;

use crate::print_to_output;

pub fn get_io_builtins() -> Vec<Builtin> {
    vec![Builtin {
        name: "print",
        args: INFINITY as usize,
        function: |env: &RefEnv,
                   _: &Token,
                   args: &Vec<Expression>|
         -> Result<Value, RuntimeError> {
            for arg in args {
                let value = arg.evaluate_expression(env)?;
                print_to_output(&value.to_string())
            }

            Ok(Value::Null)
        },
    }]
}
