use std::f64::INFINITY;

use interpreter::{
    builtin::Builtin, environment::RefEnv, error::RuntimeError, value::Value, EvalExpr,
};
use lexer::tokens::Token;
use parser::ast::Expression;

use crate::print_to_output;

#[rustfmt::skip]
pub fn get_io_builtins() -> Vec<Builtin> {
    vec![
        Builtin::new("print", INFINITY as usize, io_print),
    ]
}

fn io_print(env: &RefEnv, _: &Token, args: &Vec<Expression>) -> Result<Value, RuntimeError> {
    for arg in args {
        let value = arg.evaluate_expression(env)?;
        print_to_output(&value.to_string())
    }

    Ok(Value::Null)
}
