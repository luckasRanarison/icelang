use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl Value {
    pub fn get_type(&self) -> String {
        let value_type = match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Null => "null",
        };

        value_type.to_string()
    }

    pub fn stringify(&self) -> String {
        match self {
            Value::Number(value) => {
                let mut s = value.to_string();

                if s.ends_with(".0") {
                    s.truncate(s.len() - 2)
                }

                s
            }
            Value::String(value) => format!("\"{}\"", value),
            Value::Boolean(value) => match value {
                true => String::from("true"),
                false => String::from("false"),
            },
            Value::Null => String::from("null"),
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }
}

impl Mul for Value {
    type Output = Option<Value>;

    fn mul(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a * b)),
            _ => None,
        }
    }
}

impl Div for Value {
    type Output = Option<Value>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a / b)),
            _ => None,
        }
    }
}

impl Sub for Value {
    type Output = Option<Value>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a - b)),
            _ => None,
        }
    }
}

impl Add for Value {
    type Output = Option<Value>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a + b)),
            (Value::String(a), Value::Number(b)) => Some(Value::String(format!("{}{}", a, b))),
            (Value::Number(a), Value::String(b)) => Some(Value::String(format!("{}{}", a, b))),
            (Value::String(a), Value::String(b)) => Some(Value::String(format!("{}{}", a, b))),
            _ => None,
        }
    }
}
